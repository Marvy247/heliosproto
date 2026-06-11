# @heliosproto/extension

Browser extension build of Helios Wallet (Chrome / Firefox, Manifest V3).

> **Status:** scaffolding only. The MV3 manifest is in place; bundler, popup UI, and the `window.stellar` provider injection are not yet implemented.

## Planned structure

- `src/popup/` — the wallet UI (React, reused from `@heliosproto/app`)
- `src/background.ts` — MV3 service worker
- `src/content/` — content scripts that inject `window.stellar` into pages
- `src/provider/` — the wallet provider implementation
- `build/` — bundler config (likely Vite or wxt)

See [ARCHITECTURE.md](../../ARCHITECTURE.md) for the full design.
