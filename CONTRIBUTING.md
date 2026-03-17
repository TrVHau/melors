# Contributing to melors

Thanks for taking the time to contribute.

## Ways to Contribute

- Report bugs
- Propose features
- Improve docs
- Submit code changes
- Review open pull requests

## Development Workflow

1. Fork the repository.
2. Clone your fork locally.
3. Create a branch for your change.

```bash
git checkout -b feat/short-description
```

4. Implement your change with small, focused commits.
5. Run checks locally before opening a PR.

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo check
```

For release-readiness or documentation-impact changes, also run:

```bash
scripts/release-gate.sh
scripts/publish-check.sh
```

6. Push the branch and open a Pull Request.

## Pull Request Guidelines

- Keep PR scope focused on one concern.
- Add or update docs when behavior changes.
- Explain the why, not just the what.
- Include testing notes in the PR description.
- Link related issues when applicable.
- Fill the docs impact and release readiness checklist in the PR template.

## Commit Message Suggestions

Use clear, concise commit messages:

- `feat: add auto-next when track ends`
- `fix: avoid panic when queue is empty`
- `docs: update setup and contribution guide`

## Code Style

- Follow Rust formatting (`cargo fmt`).
- Prefer readable, explicit code over clever shortcuts.
- Keep module boundaries clear (`core`, `services`, `features`, `ui`, `app`).

## Issue Reporting

When reporting bugs, include:

- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version)
- Relevant logs or screenshots

## Community Expectations

Please be respectful and constructive in discussions and reviews.
