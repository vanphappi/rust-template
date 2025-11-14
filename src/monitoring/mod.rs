// Monitoring module - Metrics and Tracing

#[cfg(feature = "observability-tracing")]
pub mod tracing;

#[cfg(feature = "observability-metrics")]
pub mod metrics;

// Re-export commonly used items
#[cfg(feature = "observability-tracing")]
pub use self::tracing::{init_tracing, shutdown_tracing};

#[cfg(feature = "observability-metrics")]
pub use self::metrics::{init_metrics, record_request, record_error};

