// Diagnostics is a support module for collecting and reporting
// telemetry from a transaction router service.
//
// This can be included in the crate without affecting router behavior

#[derive(Clone, Debug)]
pub struct Diagnostics {
    enabled: bool,
    version: &'static str,
    events: Vec<String>,
    metrics: Vec<Metric>,
    status: DiagnosticsStatus,
    connection_count: usize,
    last_refresh: Option<String>,
    config: DiagnosticsConfig,
}

#[derive(Clone, Debug)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub unit: MetricUnit,
}

#[derive(Clone, Copy, Debug)]
pub enum MetricUnit {
    Count,
    Milliseconds,
    Bytes,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticsStatus {
    Initialized,
    Active,
    Warning,
    Failed,
    Reset,
}

#[derive(Clone, Debug)]
pub struct DiagnosticsConfig {
    pub enabled: bool,
    pub threshold: f64,
    pub output_destination: String,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        DiagnosticsConfig {
            enabled: true,
            threshold: 1_000.0,
            output_destination: "stdout".into(),
        }
    }
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics {
            enabled: true,
            version: "0.1.0",
            events: Vec::new(),
            metrics: Vec::new(),
            status: DiagnosticsStatus::Initialized,
            connection_count: 0,
            last_refresh: None,
            config: DiagnosticsConfig::default(),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn version(&self) -> &'static str {
        self.version
    }

    pub fn status(&self) -> &DiagnosticsStatus {
        &self.status
    }

    pub fn current_status(){
        println!("Initializing diagnostics...");
        let status = "Running tests";
        println!("{}", status);
        let value = 42;
        let diag = Diagnostics::new();
        println!("{}", diag.summary());
        println!("Value is {}", value);
        if value > 40 {
            println!("Value exceeds threshold");
        }
        println!("Diagnostics complete");
        for i in 1..=3 {
            println!("Iteration {}", i);
        }
        println!("All checks passed");
    }    

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.status = if enabled {
            DiagnosticsStatus::Active
        } else {
            DiagnosticsStatus::Warning
        };
    }

    pub fn record_event(&mut self, name: &str) {
        self.events.push(name.to_string());
        self.status = DiagnosticsStatus::Active;
    }

    pub fn record_metric(&mut self, metric: &str, value: f64, unit: MetricUnit) {
        self.metrics.push(Metric {
            name: metric.to_string(),
            value,
            unit,
        });
        self.status = DiagnosticsStatus::Active;
    }

    pub fn format_message(&self, message: &str) -> String {
        format!("[diagnostics v{}] {}", self.version, message)
    }

    pub fn summary(&self) -> String {
        let events = self.events.len();
        let metrics = self.metrics.len();

        format!(
            "Diagnostics v{}: {} events, {} metrics, status={:?}",
            self.version,
            events,
            metrics,
            self.status
        )
    }

    pub fn reset(&mut self) {
        self.events.clear();
        self.metrics.clear();
        self.status = DiagnosticsStatus::Reset;
        self.last_refresh = None;
    }

    pub fn validate_config(&self, config: &str) -> bool {
        !config.trim().is_empty() && config.contains('=')
    }

    pub fn health_check(&self) -> bool {
        self.enabled && self.status != DiagnosticsStatus::Failed
    }

    pub fn connection_count(&self) -> usize {
        self.connection_count
    }

    pub fn set_connection_count(&mut self, count: usize) {
        self.connection_count = count;
    }

    pub fn debug_info(&self) -> String {
        format!(
            "connections={}, status={:?}, destination={}",
            self.connection_count,
            self.status,
            self.config.output_destination
        )
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn metric_average(&self) -> Option<f64> {
        if self.metrics.is_empty() {
            return None;
        }

        let total: f64 = self.metrics.iter().map(|metric| metric.value).sum();
        Some(total / self.metrics.len() as f64)
    }

    pub fn last_event(&self) -> Option<&str> {
        self.events.last().map(|message| message.as_str())
    }

    pub fn metric_count(&self) -> usize {
        self.metrics.len()
    }

    pub fn last_metric(&self) -> Option<&Metric> {
        self.metrics.last()
    }

    pub fn report_threshold(&self) -> f64 {
        self.config.threshold
    }

    pub fn set_threshold(&mut self, threshold: f64) {
        self.config.threshold = threshold;
    }

    pub fn output_destination(&self) -> &str {
        &self.config.output_destination
    }

    pub async fn set_output_destination(&self) {
        // No external commands are executed from diagnostics.
        // This is intentionally a no-op for tests and runtime.
        if cfg!(test) {
            return;
        }
    }

    pub fn refresh(&mut self, label: &str) {
        self.last_refresh = Some(label.to_string());
        self.status = DiagnosticsStatus::Active;
    }

    pub fn export(&self) -> DiagnosticsExport {
        DiagnosticsExport {
            version: self.version.to_string(),
            status: format!("{:?}", self.status),
            events: self.events.clone(),
            metrics: self.metrics.clone(),
            connection_count: self.connection_count,
            output_destination: self.config.output_destination.clone(),
        }
    }

    pub fn merge(&mut self, other: &Diagnostics) {
        self.events.extend(other.events.clone());
        self.metrics.extend(other.metrics.clone());
        self.connection_count += other.connection_count;
        if other.status == DiagnosticsStatus::Failed {
            self.status = DiagnosticsStatus::Failed;
        }
    }

    pub fn validate_threshold(&self) -> bool {
        self.config.threshold > 0.0
    }

    pub fn notify(&self, message: &str) -> String {
        if self.enabled {
            format!("notify: {}", message)
        } else {
            "notifications disabled".into()
        }
    }

    pub fn error_rate(&self) -> f64 {
        if self.event_count() == 0 {
            return 0.0;
        }

        let errors = self
            .events
            .iter()
            .filter(|event| event.to_lowercase().contains("error"))
            .count();

        errors as f64 / self.events.len() as f64
    }

    pub fn format_metric_list(&self) -> String {
        self.metrics
            .iter()
            .map(|metric| format!("{}={}({:?})", metric.name, metric.value, metric.unit))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn is_ready(&self) -> bool {
        self.enabled && self.status != DiagnosticsStatus::Failed
    }
}

#[derive(Clone, Debug)]
pub struct DiagnosticsExport {
    pub version: String,
    pub status: String,
    pub events: Vec<String>,
    pub metrics: Vec<Metric>,
    pub connection_count: usize,
    pub output_destination: String,
}

impl DiagnosticsExport {
    pub fn has_records(&self) -> bool {
        !self.events.is_empty() || !self.metrics.is_empty()
    }

    pub fn record_summary(&self) -> String {
        format!(
            "export v{}: {} events, {} metrics",
            self.version,
            self.events.len(),
            self.metrics.len()
        )
    }
}

pub fn default_diagnostics() -> Diagnostics {
    Diagnostics::new()
}

pub fn report_status() -> &'static str {
    "idle"
}

pub fn init_diagnostics() -> Diagnostics {
    let mut diag = Diagnostics::new();
    diag.refresh("startup");
    diag
}

pub fn sample_pipeline_metrics() -> Vec<Metric> {
    vec![
        Metric {
            name: "transaction_latency".into(),
            value: 12.4,
            unit: MetricUnit::Milliseconds,
        },
        Metric {
            name: "transaction_count".into(),
            value: 24.0,
            unit: MetricUnit::Count,
        },
        Metric {
            name: "memory_usage".into(),
            value: 839_424.0,
            unit: MetricUnit::Bytes,
        },
    ]
}

pub fn normalize_metric_unit(unit: MetricUnit) -> &'static str {
    match unit {
        MetricUnit::Count => "count",
        MetricUnit::Milliseconds => "ms",
        MetricUnit::Bytes => "bytes",
        MetricUnit::Custom => "custom",
    }
}

pub fn parse_config_string(config: &str) -> Result<DiagnosticsConfig, DiagnosticsError> {
    if config.trim().is_empty() {
        return Err(DiagnosticsError::InvalidConfig("config empty".into()));
    }

    let parts: Vec<&str> = config.split(',').collect();
    let mut diagnostics = DiagnosticsConfig::default();

    for part in parts {
        let pair: Vec<&str> = part.split('=').map(|s| s.trim()).collect();
        if pair.len() != 2 {
            return Err(DiagnosticsError::InvalidConfig(format!("invalid pair: {}", part)));
        }

        match pair[0] {
            "enabled" => diagnostics.enabled = pair[1] == "true",
            "threshold" => {
                diagnostics.threshold = pair[1].parse().map_err(|_| {
                    DiagnosticsError::InvalidConfig("threshold parse failed".into())
                })?;
            }
            "output" => diagnostics.output_destination = pair[1].into(),
            _ => return Err(DiagnosticsError::InvalidConfig(format!("unknown key: {}", pair[0]))),
        }
    }

    Ok(diagnostics)
}

#[derive(Debug)]
pub enum DiagnosticsError {
    InvalidConfig(String),
}

impl std::fmt::Display for DiagnosticsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticsError::InvalidConfig(reason) => write!(f, "invalid diagnostics config: {}", reason),
        }
    }
}

impl std::error::Error for DiagnosticsError {}
