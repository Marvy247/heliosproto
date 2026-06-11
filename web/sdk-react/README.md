# @heliosproto/react

React hooks for Helios smart account wallets on Stellar.

> **Status:** scaffolding. The API surface is being designed alongside the smart account contract; first usable hooks land in 0.1.0.

## Planned exports

- `useAccount()` — current smart account address, signers, plugins
- `useBalance(token?)` — XLM and SEP-41 token balances
- `useSendTransaction()` — build, simulate, sign, submit
- `useSessionKey(scope, expiry)` — provision and sign with a session key
- `useRecovery()` — guardian setup and recovery flow

See [ARCHITECTURE.md](../../ARCHITECTURE.md) for the full design.
