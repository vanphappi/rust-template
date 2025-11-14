use prometheus::{
    Encoder, HistogramVec, IntCounterVec, IntGaugeVec, Registry, TextEncoder,
};
use std::sync::Arc;

/// Metrics collector cho Prometheus
pub struct MetricsCollector {
    registry: Registry,
    pub http_requests_total: IntCounterVec,
    pub http_request_duration_seconds: HistogramVec,
    pub http_requests_in_flight: IntGaugeVec,
    pub active_connections: IntGaugeVec,
}

impl MetricsCollector {
    pub fn new() -> Arc<Self> {
        let registry = Registry::new();

        // HTTP request counter
        let http_requests_total = IntCounterVec::new(
            prometheus::opts!("http_requests_total", "Total HTTP requests"),
            &["method", "endpoint", "status"],
        )
        .unwrap();

        // HTTP request duration histogram
        let http_request_duration_seconds = HistogramVec::new(
            prometheus::histogram_opts!(
                "http_request_duration_seconds",
                "HTTP request duration in seconds"
            ),
            &["method", "endpoint"],
        )
        .unwrap();

        // In-flight requests gauge
        let http_requests_in_flight = IntGaugeVec::new(
            prometheus::opts!("http_requests_in_flight", "HTTP requests in flight"),
            &["method", "endpoint"],
        )
        .unwrap();

        // Active connections gauge
        let active_connections = IntGaugeVec::new(
            prometheus::opts!("active_connections", "Active connections"),
            &["type"],
        )
        .unwrap();

        // Register all metrics
        registry.register(Box::new(http_requests_total.clone())).unwrap();
        registry.register(Box::new(http_request_duration_seconds.clone())).unwrap();
        registry.register(Box::new(http_requests_in_flight.clone())).unwrap();
        registry.register(Box::new(active_connections.clone())).unwrap();

        Arc::new(Self {
            registry,
            http_requests_total,
            http_request_duration_seconds,
            http_requests_in_flight,
            active_connections,
        })
    }

    /// Export metrics in Prometheus format
    pub fn export(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new().as_ref().clone()
    }
}

impl Clone for MetricsCollector {
    fn clone(&self) -> Self {
        Self {
            registry: Registry::new(),
            http_requests_total: self.http_requests_total.clone(),
            http_request_duration_seconds: self.http_request_duration_seconds.clone(),
            http_requests_in_flight: self.http_requests_in_flight.clone(),
            active_connections: self.active_connections.clone(),
        }
    }
}
