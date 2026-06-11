# Architecture

This document describes the technical architecture of `heliosproto` and the reasoning behind it. It is the canonical reference for contributors and the basis for the project's [Stellar Community Fund](https://stellar.gitbook.io/scf-handbook/) application.

## 1. Problem statement

Soroban introduced C-addresses (smart contract accounts) on Stellar, but the wallet ecosystem still treats them as a secondary feature. Existing wallets — Freighter, Lobstr, xBull — are G-address-first; they sign with classic keys, then optionally interact with Soroban contracts. The Stellar Community Fund's v7.0 RFP track explicitly identifies two adoption blockers:

1. **G→C funding gap.** Users cannot fund a C-address without first acquiring and managing a G-address. This breaks the new-user flow.
2. **Lack of C-address parity tooling.** No production wallet exists that treats the smart account as the primary identity at full parity with Freighter (token display, history, signing, dApp connectivity).

`heliosproto` addresses both, plus a longer tail of UX gains that smart accounts enable but classic accounts cannot.

## 2. Design principles

**P1. Smart account is the identity.** Onboarding produces a C-address by default. A G-address is only ever created as an internal funding helper, hidden from the user, and used once.

**P2. No seed phrase in the default flow.** Passkey (WebAuthn) is the default signing surface. Seed-phrase export is opt-in for power users.

**P3. Plugins are first-class.** Policies (session keys, spending limits, recovery, allowlists) are deployed as composable plugin contracts, not hardcoded into one monolithic account contract. New policies can ship without forking the core.

**P4. Three independent pillars.** Contracts, web, and backend each run as a separate workstream with its own build, tests, and contributors. A frontend contributor never needs to touch Rust; a backend contributor never needs to touch React.

**P5. Trust nothing offchain.** The relayer, indexer, and recovery services are convenience layers. Every state-changing action is verified onchain. Run-your-own is the default deployment story; the hosted services are optional.

**P6. Stellar SEPs over reinvention.** Reuse SEP-10 (web auth), SEP-24 (interactive deposit/withdraw), SEP-31 (cross-border), SEP-41 (Soroban tokens), SEP-30 (recovery server) wherever they apply.

## 3. Pillar 1 — Contracts (Rust + Soroban SDK)

The onchain layer. All sit in `contracts/`.

### 3.1 Smart account (`smart-account/`)

The core account contract. Responsibilities:
- Holds the **authorized signers** set (single signer, multisig, passkey credential IDs)
- Maintains a per-account **nonce**
- Verifies signatures against the authorized set
- Executes **batched calls** atomically
- Maintains the **installed plugin set** (addresses + flags)
- Exposes a `__check_auth` entrypoint that delegates policy decisions to installed plugins

Storage layout (instance + persistent):
- `signers: Vec<Signer>` where `Signer = { kind: Ed25519 | Secp256r1, pubkey }`
- `nonce: u64`
- `plugins: Map<PluginAddress, PluginFlags>`
- `threshold: u32` (for multisig)

### 3.2 Plugins (`plugins/*`)

Each plugin is a separate Soroban contract that implements a common policy interface:

```rust
fn check(env: Env, account: Address, call: CallContext) -> CheckResult;
```

Plugins inspect the proposed action and return `Allow`, `Deny`, or `Defer`. The smart account aggregates plugin results: any `Deny` aborts; all `Allow` (or `Defer` with at least one explicit `Allow`) proceeds.

Initial plugin set:
- **session-keys** — temporary signer with expiry, scope (allowed contracts), and spending bounds
- **social-recovery** — M-of-N guardian recovery to rotate the primary signer after a timelock
- **spending-limits** — per-token, per-period caps (e.g., $100/day in USDC)
- **time-lock** — delay between request and execution for high-value actions
- **allowlist** — restrict outbound calls to a whitelist of contract addresses

### 3.3 Paymaster (`paymaster/`)

Contract that sponsors transaction fees for users. Holds an XLM balance, exposes `sponsor(user, call_hash, signature)` that verifies an off-chain sponsor signature and pays the fee. The off-chain relayer (see Pillar 3) decides what to sponsor.

### 3.4 Factory (`factory/`)

Deterministic deployment helper. Given a user's passkey credential ID, the factory deploys a smart-account contract at a deterministic address derived via `create_contract_with_salt(passkey_id)`. This lets the wallet display the account address before the first transaction is signed (CREATE2-style).

## 4. Pillar 2 — Web (TypeScript + Next.js 15)

The user-facing layer. All in `web/`.

### 4.1 `web/app/` — wallet web app (PWA)

A Next.js 15 App Router application. Major routes:

| Route | Purpose |
|---|---|
| `/onboard` | Passkey enrollment, account creation via factory, G→C funding flow |
| `/` | Dashboard: balances (classic + SEP-41 tokens), recent activity |
| `/send` | Multi-step send flow with simulation preview |
| `/receive` | QR code, copyable address, payment links |
| `/history` | Paginated transaction history with decoded events |
| `/anchor` | SEP-24 interactive deposit/withdraw flows |
| `/settings/plugins` | Manage installed plugins: session keys, limits, recovery setup |
| `/settings/security` | Add signers, change threshold, view paired devices |
| `/connect` | dApp connector — WalletConnect-compatible session handshake |

State: TanStack Query + Zustand. Styling: Tailwind 4 + shadcn/ui. Forms: react-hook-form + zod. Network: published `sdk-ts` package.

### 4.2 `web/extension/` — browser extension

Same React component library as the web app, packaged as a Manifest V3 extension (Chrome + Firefox). Communicates with dApps via the standard Stellar wallet provider injection (`window.stellar`).

### 4.3 `web/sdk-react/` — React hooks package

Publishable `@heliosproto/react` package: `useAccount`, `useBalance`, `useSendTransaction`, `useSessionKey`, `useRecovery`. Used by Helios Wallet itself and by third-party dApps that want to integrate Helios accounts.

## 5. Pillar 3 — Backend (Python 3.12 + FastAPI)

Convenience services. All in `backend/`. **Every backend service is optional** — the wallet works against vanilla Soroban RPC without any of them — but they materially improve UX.

### 5.1 `indexer/`

Subscribes to Soroban RPC events, decodes them against known contract ABIs (SEP-41 tokens, plugins, paymaster), persists to Postgres. Backfills history on first observation of a new address. Powers fast `/history` rendering and search.

### 5.2 `relayer/`

The G→C funding bridge service. When a user onboards:
1. Frontend creates a deterministic smart-account address via the factory
2. Frontend tells the relayer: "fund this C-address with X USDC"
3. Relayer obtains the funds (CEX withdrawal, anchor deposit, or its own funding pool) and submits the on-chain transfer
4. Relayer optionally also acts as **paymaster** for the first N transactions, so the user never needs to hold XLM

The relayer's economic model is configurable per deployment: subsidize via a treasury, charge a fee at off-ramp, or both.

### 5.3 `recovery/`

Coordinates social recovery. Stores guardian contact info (email/phone, encrypted), routes recovery requests to guardians out-of-band, collects guardian signatures, and submits them to the on-chain `social-recovery` plugin. Implements [SEP-30](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0030.md) where applicable.

### 5.4 `notifications/`

WebSocket + push (FCM/APNS) + email. Subscribes to indexer events, filters per-user, fans out notifications: incoming payment, session key used, recovery initiated, plugin policy violation.

### 5.5 `anchors/`

SEP-24 / SEP-31 / SEP-10 adapter framework. Lets the wallet UI list available on/off-ramps (Yellowcard, Linkio, MoneyGram, etc.) and proxy the interactive deposit/withdraw flow. Especially relevant for Naira ↔ USDC flows.

### 5.6 `prices/`

Polls [Reflector](https://reflector.network/), Chainlink Data Feeds, and other Stellar-deployed oracles, caches normalized USD prices for all SEP-41 tokens, exposes them to the frontend portfolio view.

### 5.7 `api/`

The gateway. REST + GraphQL. Authn via SEP-10. Aggregates indexer, prices, notifications subscriptions into a single endpoint for the web app and extension.

### 5.8 `workers/`

Celery workers for: indexer backfills, notification fanout, scheduled price polling, recovery timeout sweepers, anchor status polling.

## 6. SDK packages

- **`sdk-ts/`** — framework-agnostic TypeScript SDK. Wraps `@stellar/stellar-sdk` with `heliosproto`-specific helpers: account creation, plugin installation, session-key signing, recovery requests.
- **`sdk-py/`** — Python SDK with the same surface, for backend integrators (anchors, dApps, bots).
- **`web/sdk-react/`** — React hooks layer on top of `sdk-ts`.

## 7. Deployment topology

```
[ Browser / Mobile ]
        │
        ▼
[ Helios Wallet (web app or extension) ]
        │
        ├──► Soroban RPC node (direct, for state-changing tx)
        │
        └──► heliosproto backend API
                 │
                 ├──► Postgres (indexed state)
                 ├──► Redis (cache, pubsub)
                 ├──► Celery workers
                 └──► External SEP anchors / oracles / push providers
```

**Self-hostable.** The default deployment is a single Docker Compose stack (Postgres + Redis + FastAPI + Celery + indexer). Hosted instance is convenience-only; users with sovereignty requirements can run their own.

## 8. SCF RFP deliverable mapping

| SCF RFP requirement | heliosproto component |
|---|---|
| G→C seamless bridge | `backend/relayer/` + `web/app/onboard/` |
| Funding from G-address, CEX withdrawal, off-ramp | `backend/anchors/` + relayer integrations |
| Production wallet at Freighter parity | `web/app/` + `web/extension/` |
| Show all tokens held | `backend/indexer/` + dashboard route |
| Transfer history | `backend/indexer/` + `/history` route |

Each is a milestone in the SCF v7.0 funding tranches (10% / 20% / 30% / 40%).

## 9. Open questions

These are not yet decided and will be resolved as we build:

- **Account model:** single canonical signer-plus-plugins vs. modular account where even the signer is a plugin (ERC-7579-style)
- **Passkey storage:** WebAuthn credential ID directly on-chain, or hash with off-chain credential mapping
- **Plugin upgradeability:** immutable plugin contracts (forces redeployment + migration) vs. proxied plugins (faster iteration but trust assumptions)
- **Cross-chain:** whether to integrate Squid / Allbridge in v1 or defer to a separate repo

Discussions happen in GitHub Discussions; resolutions are captured in `docs/decisions/`.
