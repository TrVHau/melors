# Docs Sync Checklist Policy (Release Gate)

Use this as the minimum docs policy when preparing release candidates:

- README reflects user-visible behavior changes for this release candidate.
- CONTRIBUTING release workflow guidance is aligned with current scripts.
- Release runbook is updated for new operational/release steps.
- PR includes docs impact section and checklist evidence.

Automation note:
- `scripts/release-gate.sh` enforces that code/config/runtime changes must include docs changes in the same diff range.
