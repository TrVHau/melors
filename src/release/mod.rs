#![allow(dead_code)]

use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseStatus {
    Draft,
    ReadyForApproval,
    Approved,
    Published,
    RolledBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollbackStatus {
    NotRequiredPrePublish,
    Ready,
    Triggered,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactEntry {
    pub path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactPackage {
    pub candidate_id: String,
    pub artifacts: Vec<ArtifactEntry>,
    pub release_notes_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityReport {
    pub passed: bool,
    pub reasons: Vec<String>,
}

pub fn validate_artifact_package(package: &ArtifactPackage) -> IntegrityReport {
    let mut reasons = Vec::new();
    let mut paths = BTreeSet::new();

    if package.candidate_id.trim().is_empty() {
        reasons.push(String::from("candidate_id is required"));
    }
    if package.release_notes_ref.trim().is_empty() {
        reasons.push(String::from("release_notes_ref is required"));
    }
    if package.artifacts.is_empty() {
        reasons.push(String::from("artifact set cannot be empty"));
    }

    for artifact in &package.artifacts {
        if artifact.path.trim().is_empty() {
            reasons.push(String::from("artifact path cannot be empty"));
        }
        if artifact.checksum.trim().is_empty() {
            reasons.push(format!("artifact checksum missing for {}", artifact.path));
        }
        if !paths.insert(artifact.path.clone()) {
            reasons.push(format!("duplicate artifact path: {}", artifact.path));
        }
    }

    IntegrityReport {
        passed: reasons.is_empty(),
        reasons,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsChecklist {
    pub required_items: Vec<String>,
    pub completed_items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsSyncVerdict {
    pub passed: bool,
    pub missing_items: Vec<String>,
}

pub fn evaluate_docs_sync(checklist: &DocsChecklist) -> DocsSyncVerdict {
    let completed: BTreeSet<&str> = checklist
        .completed_items
        .iter()
        .map(String::as_str)
        .collect();
    let mut missing = Vec::new();

    for item in &checklist.required_items {
        if !completed.contains(item.as_str()) {
            missing.push(item.clone());
        }
    }

    DocsSyncVerdict {
        passed: missing.is_empty(),
        missing_items: missing,
    }
}

pub fn parse_docs_checklist_markdown(content: &str) -> DocsChecklist {
    let mut required_items = Vec::new();
    let mut completed_items = Vec::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if let Some(rest) = line.strip_prefix("- [x] ") {
            let item = rest.trim().to_string();
            required_items.push(item.clone());
            completed_items.push(item);
        } else if let Some(rest) = line.strip_prefix("- [ ] ") {
            required_items.push(rest.trim().to_string());
        }
    }

    DocsChecklist {
        required_items,
        completed_items,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalEvent {
    pub candidate_id: String,
    pub stage: String,
    pub approver: String,
    pub timestamp_utc: String,
}

pub fn append_approval_event(log: &mut Vec<ApprovalEvent>, event: ApprovalEvent) {
    log.push(event);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionInputs {
    pub quality_gate_passed: bool,
    pub integrity_passed: bool,
    pub docs_sync_passed: bool,
    pub approval_count: usize,
    pub rollback_status: RollbackStatus,
}

pub fn can_transition(
    from: ReleaseStatus,
    to: ReleaseStatus,
    inputs: &TransitionInputs,
) -> Result<(), String> {
    match (from, to) {
        (ReleaseStatus::Draft, ReleaseStatus::ReadyForApproval) => {
            if !inputs.quality_gate_passed {
                return Err(String::from(
                    "quality gate must pass before ReadyForApproval",
                ));
            }
            if !inputs.integrity_passed {
                return Err(String::from(
                    "artifact integrity must pass before ReadyForApproval",
                ));
            }
            if !inputs.docs_sync_passed {
                return Err(String::from("docs sync must pass before ReadyForApproval"));
            }
            Ok(())
        }
        (ReleaseStatus::ReadyForApproval, ReleaseStatus::Approved) => {
            if inputs.approval_count == 0 {
                return Err(String::from(
                    "at least one explicit approval event is required",
                ));
            }
            Ok(())
        }
        (ReleaseStatus::Approved, ReleaseStatus::Published) => {
            if !inputs.quality_gate_passed || !inputs.integrity_passed || !inputs.docs_sync_passed {
                return Err(String::from("all gates must remain passing before publish"));
            }
            if matches!(
                inputs.rollback_status,
                RollbackStatus::Triggered | RollbackStatus::Completed
            ) {
                return Err(String::from(
                    "cannot publish while rollback is active/completed",
                ));
            }
            Ok(())
        }
        (ReleaseStatus::Published, ReleaseStatus::RolledBack) => Ok(()),
        _ => Err(format!("invalid transition: {:?} -> {:?}", from, to)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseSummary {
    pub candidate_id: String,
    pub release_status: ReleaseStatus,
    pub quality_gate_passed: bool,
    pub integrity_passed: bool,
    pub docs_sync_passed: bool,
    pub approval_count: usize,
    pub rollback_status: RollbackStatus,
    pub rollback_owner: String,
    pub rollback_next_step: String,
}

impl ReleaseSummary {
    pub fn to_markdown(&self) -> String {
        format!(
            "# Release Summary\n\n- Candidate ID: {}\n- Release Status: {:?}\n- Quality Gate: {}\n- Integrity Gate: {}\n- Docs Sync Gate: {}\n- Approval Events: {}\n- Rollback Status: {:?}\n- Rollback Owner: {}\n- Rollback Next Step: {}\n",
            self.candidate_id,
            self.release_status,
            self.quality_gate_passed,
            self.integrity_passed,
            self.docs_sync_passed,
            self.approval_count,
            self.rollback_status,
            self.rollback_owner,
            self.rollback_next_step
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrity_report_fails_when_checksum_or_release_notes_missing() {
        let package = ArtifactPackage {
            candidate_id: String::from("rc-001"),
            artifacts: vec![ArtifactEntry {
                path: String::from("target/release/melors"),
                checksum: String::new(),
            }],
            release_notes_ref: String::new(),
        };

        let report = validate_artifact_package(&package);
        assert!(!report.passed);
        assert!(
            report
                .reasons
                .iter()
                .any(|r| r.contains("release_notes_ref"))
        );
        assert!(report.reasons.iter().any(|r| r.contains("checksum")));
    }

    #[test]
    fn docs_sync_blocks_when_required_items_missing() {
        let checklist = DocsChecklist {
            required_items: vec![
                String::from("README release section"),
                String::from("release runbook"),
            ],
            completed_items: vec![String::from("README release section")],
        };

        let verdict = evaluate_docs_sync(&checklist);
        assert!(!verdict.passed);
        assert_eq!(verdict.missing_items, vec![String::from("release runbook")]);
    }

    #[test]
    fn parse_docs_checklist_reads_checkbox_state() {
        let markdown = "- [x] README release section\n- [ ] PR template docs impact section\n";
        let parsed = parse_docs_checklist_markdown(markdown);

        assert_eq!(parsed.required_items.len(), 2);
        assert_eq!(
            parsed.completed_items,
            vec![String::from("README release section")]
        );
    }

    #[test]
    fn transition_requires_quality_integrity_docs_before_ready_for_approval() {
        let inputs = TransitionInputs {
            quality_gate_passed: true,
            integrity_passed: false,
            docs_sync_passed: true,
            approval_count: 0,
            rollback_status: RollbackStatus::NotRequiredPrePublish,
        };

        let result = can_transition(
            ReleaseStatus::Draft,
            ReleaseStatus::ReadyForApproval,
            &inputs,
        );
        assert!(result.is_err());
    }

    #[test]
    fn transition_requires_explicit_approval_event() {
        let inputs = TransitionInputs {
            quality_gate_passed: true,
            integrity_passed: true,
            docs_sync_passed: true,
            approval_count: 0,
            rollback_status: RollbackStatus::Ready,
        };

        let result = can_transition(
            ReleaseStatus::ReadyForApproval,
            ReleaseStatus::Approved,
            &inputs,
        );
        assert!(result.is_err());
    }

    #[test]
    fn publish_transition_passes_with_all_inputs_green() {
        let inputs = TransitionInputs {
            quality_gate_passed: true,
            integrity_passed: true,
            docs_sync_passed: true,
            approval_count: 1,
            rollback_status: RollbackStatus::Ready,
        };

        let result = can_transition(ReleaseStatus::Approved, ReleaseStatus::Published, &inputs);
        assert!(result.is_ok());
    }

    #[test]
    fn approval_logger_appends_events_immutably() {
        let mut log = Vec::new();
        append_approval_event(
            &mut log,
            ApprovalEvent {
                candidate_id: String::from("rc-002"),
                stage: String::from("ReadyForApproval->Approved"),
                approver: String::from("maintainer"),
                timestamp_utc: String::from("2026-03-17T00:00:00Z"),
            },
        );
        append_approval_event(
            &mut log,
            ApprovalEvent {
                candidate_id: String::from("rc-002"),
                stage: String::from("Approved->Published"),
                approver: String::from("maintainer"),
                timestamp_utc: String::from("2026-03-17T00:05:00Z"),
            },
        );
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].stage, "ReadyForApproval->Approved");
        assert_eq!(log[1].stage, "Approved->Published");
    }

    #[test]
    fn release_summary_contains_required_communication_fields() {
        let summary = ReleaseSummary {
            candidate_id: String::from("rc-003"),
            release_status: ReleaseStatus::Published,
            quality_gate_passed: true,
            integrity_passed: true,
            docs_sync_passed: true,
            approval_count: 2,
            rollback_status: RollbackStatus::Ready,
            rollback_owner: String::from("release-manager"),
            rollback_next_step: String::from("Monitor first-hour error budget"),
        };
        let markdown = summary.to_markdown();
        assert!(markdown.contains("Candidate ID: rc-003"));
        assert!(markdown.contains("Rollback Owner: release-manager"));
    }
}
