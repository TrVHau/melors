#![allow(dead_code)]

pub mod diagnostics;
pub mod evaluator;
pub mod thresholds;

use diagnostics::{FailureEnvelope, QualityGateDiagnostics, SuiteDiagnostics};
use evaluator::{MetricObservation, MetricStatus, MetricVerdict, evaluate_all};
use thresholds::ThresholdRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateState {
    Ready,
    Running,
    Failed,
    Passed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuiteExecutionInput {
    pub suite: String,
    pub passed: bool,
    pub duration_ms: u128,
    pub failures: Vec<FailureEnvelope>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualityGateOutcome {
    pub state: GateState,
    pub diagnostics: QualityGateDiagnostics,
    pub metric_verdicts: Vec<MetricVerdict>,
}

pub fn local_ci_quality_gate_command() -> &'static str {
    "scripts/quality-gate.sh"
}

pub fn run_quality_gate(
    registry: &ThresholdRegistry,
    suites: &[SuiteExecutionInput],
    observations: &[MetricObservation],
) -> QualityGateOutcome {
    let mut state = GateState::Ready;
    let mut diagnostics_suites = Vec::new();

    for suite in suites {
        state = GateState::Running;
        let diagnostics = SuiteDiagnostics {
            suite: suite.suite.clone(),
            passed: suite.passed,
            failures: suite.failures.clone(),
            duration_ms: suite.duration_ms,
        };
        diagnostics_suites.push(diagnostics);

        if !suite.passed {
            state = GateState::Failed;
            break;
        }
    }

    let metric_verdicts = evaluate_all(registry, observations);
    if state != GateState::Failed
        && metric_verdicts
            .iter()
            .any(|verdict| verdict.status != MetricStatus::Pass)
    {
        state = GateState::Failed;
    }

    if state != GateState::Failed && !suites.is_empty() {
        state = GateState::Passed;
    }

    let diagnostics = QualityGateDiagnostics {
        registry_version: registry.version.clone(),
        suites: diagnostics_suites,
        overall_passed: state == GateState::Passed,
    };

    QualityGateOutcome {
        state,
        diagnostics,
        metric_verdicts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quality::thresholds::ThresholdRegistry;

    const REGISTRY: &str = r#"
version = "2026-03"

[[metrics]]
name = "quality_gate_runtime_ms"
baseline = 300000.0
tolerance_percent = 5.0
direction = "max"
"#;

    #[test]
    fn gate_fails_fast_on_first_failed_suite() {
        let registry = ThresholdRegistry::from_toml_str(REGISTRY).expect("registry");
        let suites = vec![
            SuiteExecutionInput {
                suite: String::from("regression"),
                passed: false,
                duration_ms: 12,
                failures: vec![FailureEnvelope {
                    suite: String::from("regression"),
                    case_name: String::from("queue_move"),
                    invariant: String::from("queue order remains valid"),
                    observed: String::from("out-of-order"),
                    remediation_hint: String::from("review move_queue_index bounds"),
                }],
            },
            SuiteExecutionInput {
                suite: String::from("resilience"),
                passed: true,
                duration_ms: 30,
                failures: Vec::new(),
            },
        ];
        let observations = vec![MetricObservation {
            name: String::from("quality_gate_runtime_ms"),
            value: 100_000.0,
        }];

        let outcome = run_quality_gate(&registry, &suites, &observations);
        assert_eq!(outcome.state, GateState::Failed);
        assert_eq!(outcome.diagnostics.suites.len(), 1);
    }

    #[test]
    fn gate_passes_when_suites_and_metrics_pass() {
        let registry = ThresholdRegistry::from_toml_str(REGISTRY).expect("registry");
        let suites = vec![SuiteExecutionInput {
            suite: String::from("regression"),
            passed: true,
            duration_ms: 50,
            failures: Vec::new(),
        }];
        let observations = vec![MetricObservation {
            name: String::from("quality_gate_runtime_ms"),
            value: 250_000.0,
        }];

        let outcome = run_quality_gate(&registry, &suites, &observations);
        assert_eq!(outcome.state, GateState::Passed);
        assert!(outcome.diagnostics.overall_passed);
    }
}
