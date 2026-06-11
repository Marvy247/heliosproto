## Summary

<!-- One or two sentences describing what this PR does and why. -->

Closes #<!-- issue number -->

## Pillar

<!-- Check one. PRs touching multiple pillars are usually a sign the issue was too large. -->

- [ ] `contracts/` (Rust / Soroban)
- [ ] `web/` (TypeScript / Next.js)
- [ ] `backend/` (Python / FastAPI)
- [ ] `sdk-ts/` or `sdk-py/`
- [ ] `docs/`

## What changed

<!-- Bullet list of the concrete changes. Keep it tight — the diff shows the details. -->

-
-

## Testing

<!-- How did you verify this works? Commands run, edge cases checked. -->

-

## Checklist

- [ ] I was assigned this issue through Grantfox
- [ ] PR title follows `<type>(<scope>): <subject>` (conventional commits)
- [ ] Tests added or updated for new logic
- [ ] `cargo fmt && cargo clippy -- -D warnings` clean (contracts only)
- [ ] `pnpm lint && pnpm test` clean (web only)
- [ ] `ruff check && mypy --strict && pytest` clean (backend only)
- [ ] No unrelated formatting or refactors
- [ ] Docs updated if behavior changed
