// Prometheus Metrics Integration
// Provides application metrics collection and exposition

use metrics::{counter, describe_counter, describe_histogram, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;

/// Initialize Prometheus metrics exporter
pub fn init_metrics(listen_addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    // Setup Prometheus exporter
    PrometheusBuilder::new()
        .with_http_listener(listen_addr)
        .install()?;

    // Describe metrics
    describe_counter!("http_requests_total", "Total number of HTTP requests");
    describe_counter!("http_errors_total", "Total number of HTTP errors");
    describe_histogram!("http_request_duration_seconds", "HTTP request duration in seconds");
    describe_counter!("database_queries_total", "Total number of database queries");
    describe_counter!("cache_hits_total", "Total number of cache hits");
    describe_counter!("cache_misses_total", "Total number of cache misses");

    tracing::info!("Metrics exporter started on {}", listen_addr);
    Ok(())
}

/// Record HTTP request metrics
pub fn record_request(method: &str, path: &str, status: u16, duration_ms: f64) {
    counter!("http_requests_total", "method" => method.to_string(), "path" => path.to_string(), "status" => status.to_string()).increment(1);
    histogram!("http_request_duration_seconds", "method" => method.to_string(), "path" => path.to_string()).record(duration_ms / 1000.0);
}

/// Record HTTP error metrics
pub fn record_error(method: &str, path: &str, error_type: &str) {
    counter!("http_errors_total", "method" => method.to_string(), "path" => path.to_string(), "error_type" => error_type.to_string()).increment(1);
}

/// Record database query metrics
pub fn record_database_query(query_type: &str, duration_ms: f64) {
    counter!("database_queries_total", "type" => query_type.to_string()).increment(1);
    histogram!("database_query_duration_seconds", "type" => query_type.to_string()).record(duration_ms / 1000.0);
}

/// Record cache hit
pub fn record_cache_hit(cache_type: &str) {
    counter!("cache_hits_total", "type" => cache_type.to_string()).increment(1);
}

/// Record cache miss
pub fn record_cache_miss(cache_type: &str) {
    counter!("cache_misses_total", "type" => cache_type.to_string()).increment(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_recording() {
        // Test that metrics can be recorded without panicking
        record_request("GET", "/api/users", 200, 150.0);
        record_error("POST", "/api/users", "validation_error");
        record_database_query("SELECT", 50.0);
        record_cache_hit("redis");
        record_cache_miss("redis");
    }
}

