# Web

The user-facing pillar of `heliosproto`. TypeScript, Next.js 15 App Router, React 19, Tailwind 4, shadcn/ui.

See the [Pillar 2 section of ARCHITECTURE.md](../ARCHITECTURE.md#4-pillar-2--web-typescript--nextjs-15) for the full design.

## Layout

| Path | Purpose |
|---|---|
| `app/` | Helios Wallet web app (PWA) |
| `extension/` | Chrome/Firefox MV3 browser extension |
| `sdk-react/` | Publishable `@heliosproto/react` hooks package |

## Prerequisites

- Node.js 20+
- pnpm 9+ (`corepack enable`)

## Working in this pillar

`web/` is a pnpm workspace. From `web/`:

```bash
pnpm install
pnpm --filter app dev          # run the wallet app
pnpm --filter extension build  # build the browser extension
pnpm --filter sdk-react test
```

Workspace and per-package configs land when the first package is scaffolded.
