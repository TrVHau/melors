use super::thresholds::{ThresholdDirection, ThresholdRegistry};

#[derive(Debug, Clone, PartialEq)]
pub struct MetricObservation {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricStatus {
    Pass,
    Fail,
    MissingThreshold,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricVerdict {
    pub name: String,
    pub value: f64,
    pub allowed_value: Option<f64>,
    pub status: MetricStatus,
}

pub fn evaluate_metric(
    registry: &ThresholdRegistry,
    observation: &MetricObservation,
) -> MetricVerdict {
    let Some(threshold) = registry.metric(&observation.name) else {
        return MetricVerdict {
            name: observation.name.clone(),
            value: observation.value,
            allowed_value: None,
            status: MetricStatus::MissingThreshold,
        };
    };

    let allowed_value = match threshold.direction {
        ThresholdDirection::Max => threshold.baseline * (1.0 + threshold.tolerance_percent / 100.0),
        ThresholdDirection::Min => threshold.baseline * (1.0 - threshold.tolerance_percent / 100.0),
    };

    let status = match threshold.direction {
        ThresholdDirection::Max if observation.value <= allowed_value => MetricStatus::Pass,
        ThresholdDirection::Min if observation.value >= allowed_value => MetricStatus::Pass,
        _ => MetricStatus::Fail,
    };

    MetricVerdict {
        name: observation.name.clone(),
        value: observation.value,
        allowed_value: Some(allowed_value),
        status,
    }
}

pub fn evaluate_all(
    registry: &ThresholdRegistry,
    observations: &[MetricObservation],
) -> Vec<MetricVerdict> {
    observations
        .iter()
        .map(|observation| evaluate_metric(registry, observation))
        .collect()
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

[[metrics]]
name = "scan_apply_ops_per_sec"
baseline = 100.0
tolerance_percent = 10.0
direction = "min"
"#;

    #[test]
    fn evaluate_max_threshold_with_tolerance() {
        let registry = ThresholdRegistry::from_toml_str(REGISTRY).expect("registry");
        let observation = MetricObservation {
            name: String::from("quality_gate_runtime_ms"),
            value: 314_000.0,
        };

        let verdict = evaluate_metric(&registry, &observation);
        assert_eq!(verdict.status, MetricStatus::Pass);
    }

    #[test]
    fn evaluate_min_threshold_fails_when_below_allowed() {
        let registry = ThresholdRegistry::from_toml_str(REGISTRY).expect("registry");
        let observation = MetricObservation {
            name: String::from("scan_apply_ops_per_sec"),
            value: 80.0,
        };

        let verdict = evaluate_metric(&registry, &observation);
        assert_eq!(verdict.status, MetricStatus::Fail);
    }
}
