use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ThresholdRegistry {
    pub version: String,
    pub metrics: Vec<MetricThreshold>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MetricThreshold {
    pub name: String,
    pub baseline: f64,
    pub tolerance_percent: f64,
    pub direction: ThresholdDirection,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThresholdDirection {
    Max,
    Min,
}

impl ThresholdRegistry {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read threshold registry: {}", path.display()))?;
        Self::from_toml_str(&raw)
    }

    pub fn from_toml_str(raw: &str) -> Result<Self> {
        let parsed: ThresholdRegistry =
            toml::from_str(raw).context("failed to parse threshold registry TOML")?;
        parsed.validate()?;
        Ok(parsed)
    }

    pub fn metric(&self, name: &str) -> Option<&MetricThreshold> {
        self.metrics.iter().find(|metric| metric.name == name)
    }

    fn validate(&self) -> Result<()> {
        if self.version.trim().is_empty() {
            bail!("threshold registry version cannot be empty");
        }
        if self.metrics.is_empty() {
            bail!("threshold registry must declare at least one metric");
        }
        for metric in &self.metrics {
            if metric.name.trim().is_empty() {
                bail!("metric name cannot be empty");
            }
            if metric.baseline <= 0.0 {
                bail!("metric baseline must be > 0 for {}", metric.name);
            }
            if metric.tolerance_percent < 0.0 {
                bail!("metric tolerance_percent must be >= 0 for {}", metric.name);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"
version = "2026-03"

[[metrics]]
name = "quality_gate_runtime_ms"
baseline = 300000

tolerance_percent = 5.0
direction = "max"

[[metrics]]
name = "scan_apply_ops_per_sec"
baseline = 100.0

tolerance_percent = 10.0
direction = "min"
"#;

    #[test]
    fn registry_parses_and_queries_metric() {
        let registry = ThresholdRegistry::from_toml_str(VALID).expect("valid registry");
        assert_eq!(registry.version, "2026-03");
        let metric = registry
            .metric("quality_gate_runtime_ms")
            .expect("metric exists");
        assert_eq!(metric.direction, ThresholdDirection::Max);
    }

    #[test]
    fn registry_rejects_empty_version() {
        let bad = VALID.replace("version = \"2026-03\"", "version = \"\"");
        assert!(ThresholdRegistry::from_toml_str(&bad).is_err());
    }
}
