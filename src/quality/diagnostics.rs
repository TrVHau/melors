#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureEnvelope {
    pub suite: String,
    pub case_name: String,
    pub invariant: String,
    pub observed: String,
    pub remediation_hint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuiteDiagnostics {
    pub suite: String,
    pub passed: bool,
    pub failures: Vec<FailureEnvelope>,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualityGateDiagnostics {
    pub registry_version: String,
    pub suites: Vec<SuiteDiagnostics>,
    pub overall_passed: bool,
}

impl QualityGateDiagnostics {
    pub fn first_failure(&self) -> Option<&FailureEnvelope> {
        self.suites
            .iter()
            .flat_map(|suite| suite.failures.iter())
            .next()
    }
}

pub fn sanitize_text(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_control() && ch != '\n' && ch != '\t' {
            continue;
        }
        out.push(ch);
    }
    out.replace("/home/", "<home>/")
}

pub fn sanitize_diagnostics(report: &QualityGateDiagnostics) -> QualityGateDiagnostics {
    let mut sanitized = report.clone();
    sanitized.registry_version = sanitize_text(&sanitized.registry_version);
    for suite in &mut sanitized.suites {
        suite.suite = sanitize_text(&suite.suite);
        for failure in &mut suite.failures {
            failure.suite = sanitize_text(&failure.suite);
            failure.case_name = sanitize_text(&failure.case_name);
            failure.invariant = sanitize_text(&failure.invariant);
            failure.observed = sanitize_text(&failure.observed);
            failure.remediation_hint = sanitize_text(&failure.remediation_hint);
        }
    }
    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_text_strips_control_chars_and_home_path() {
        let raw = "/home/dau/project\u{7} failed";
        let sanitized = sanitize_text(raw);
        assert_eq!(sanitized, "<home>/dau/project failed");
    }

    #[test]
    fn first_failure_returns_earliest_failure() {
        let report = QualityGateDiagnostics {
            registry_version: String::from("2026-03"),
            suites: vec![SuiteDiagnostics {
                suite: String::from("regression"),
                passed: false,
                failures: vec![FailureEnvelope {
                    suite: String::from("regression"),
                    case_name: String::from("queue_move"),
                    invariant: String::from("queue order remains valid"),
                    observed: String::from("index out of bounds"),
                    remediation_hint: String::from("verify queue index clamping"),
                }],
                duration_ms: 42,
            }],
            overall_passed: false,
        };

        let failure = report.first_failure().expect("has failure");
        assert_eq!(failure.case_name, "queue_move");
    }
}
