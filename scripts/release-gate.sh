#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

POLICY_FILE="${ROOT_DIR}/config/release-policy.toml"
CHECKLIST_FILE="${ROOT_DIR}/config/docs-sync-checklist.md"
RELEASE_NOTES_PATH="${RELEASE_NOTES_PATH:-${ROOT_DIR}/docs/README.md}"
RELEASE_APPROVER="${RELEASE_APPROVER:-local-maintainer}"
RELEASE_CANDIDATE_ID="${RELEASE_CANDIDATE_ID:-local-$(date -u +%Y%m%d%H%M%S)}"
ARTIFACT_DIR="${ROOT_DIR}/artifacts/release"
APPROVAL_LOG="${ARTIFACT_DIR}/approval-events.log"
SUMMARY_FILE="${ARTIFACT_DIR}/release-summary-${RELEASE_CANDIDATE_ID}.md"
CHECKSUM_FILE="${ARTIFACT_DIR}/checksums-${RELEASE_CANDIDATE_ID}.txt"

mkdir -p "${ARTIFACT_DIR}"

if [[ ! -f "${POLICY_FILE}" ]]; then
  echo "missing release policy: ${POLICY_FILE}" >&2
  exit 1
fi

if [[ ! -f "${CHECKLIST_FILE}" ]]; then
  echo "missing docs checklist: ${CHECKLIST_FILE}" >&2
  exit 1
fi

if [[ ! -f "${RELEASE_NOTES_PATH}" ]]; then
  echo "release notes reference not found: ${RELEASE_NOTES_PATH}" >&2
  exit 1
fi

# Docs-sync gate: if code/config/runtime behavior changed, docs must also change.
DIFF_RANGE=""
if [[ -n "${GITHUB_BASE_REF:-}" ]] && git rev-parse --verify -q "origin/${GITHUB_BASE_REF}" >/dev/null; then
  DIFF_RANGE="origin/${GITHUB_BASE_REF}...HEAD"
elif git rev-parse --verify -q HEAD~1 >/dev/null; then
  DIFF_RANGE="HEAD~1...HEAD"
fi

if [[ -n "${DIFF_RANGE}" ]]; then
  CODE_CHANGED="$(git diff --name-only "${DIFF_RANGE}" -- \
    'src/**' 'scripts/**' 'config/**' 'Cargo.toml' 'Cargo.lock' || true)"
  DOCS_CHANGED="$(git diff --name-only "${DIFF_RANGE}" -- \
    'README.md' 'CONTRIBUTING.md' 'docs/**' 'aidlc-docs/**' '.github/PULL_REQUEST_TEMPLATE.md' || true)"

  if [[ -n "${CODE_CHANGED}" ]] && [[ -z "${DOCS_CHANGED}" ]]; then
    echo "docs sync gate failed: code/config changed but no docs files changed" >&2
    echo "update README/CONTRIBUTING/docs/aidlc-docs or PR template, then rerun" >&2
    exit 1
  fi
fi

scripts/quality-gate.sh
cargo build --release

BIN_PATH="${ROOT_DIR}/target/release/melors"
if [[ ! -f "${BIN_PATH}" ]]; then
  echo "expected release artifact not found: ${BIN_PATH}" >&2
  exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
  BIN_CHECKSUM="$(sha256sum "${BIN_PATH}" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  BIN_CHECKSUM="$(shasum -a 256 "${BIN_PATH}" | awk '{print $1}')"
else
  echo "missing checksum tool: require sha256sum or shasum" >&2
  exit 1
fi

printf "%s  %s\n" "${BIN_CHECKSUM}" "target/release/melors" > "${CHECKSUM_FILE}"

APPROVAL_TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf "%s|%s|%s|%s\n" "${APPROVAL_TS}" "${RELEASE_CANDIDATE_ID}" "Approved->Published" "${RELEASE_APPROVER}" >> "${APPROVAL_LOG}"

cat > "${SUMMARY_FILE}" <<EOF
# Release Summary

- Candidate ID: ${RELEASE_CANDIDATE_ID}
- Release Status: Published
- Quality Gate: true
- Integrity Gate: true
- Docs Sync Gate: true
- Release Notes Ref: ${RELEASE_NOTES_PATH}
- Checksum Manifest: ${CHECKSUM_FILE}
- Approval Log: ${APPROVAL_LOG}
- Rollback Status: NotRequiredPrePublish
- Rollback Owner: ${RELEASE_APPROVER}
- Rollback Next Step: Monitor post-release diagnostics and trigger rollback procedure if critical regressions appear.
- Generated At (UTC): ${APPROVAL_TS}
EOF

echo "release gate passed: ${SUMMARY_FILE}"
