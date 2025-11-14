use std::time::{Duration, Instant};
use rayon::prelude::*;

/// Performance timer for measuring execution time
pub struct Timer {
    start: Instant,
    label: String,
}

impl Timer {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            label: label.into(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn log_elapsed(&self) {
        let elapsed = self.elapsed();
        tracing::info!(
            label = %self.label,
            duration_ms = elapsed.as_millis(),
            "Performance measurement"
        );
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.log_elapsed();
    }
}

/// Parallel processing utilities using Rayon
pub struct ParallelProcessor;

impl ParallelProcessor {
    /// Process items in parallel using Rayon
    pub fn process_parallel<T, F, R>(items: Vec<T>, f: F) -> Vec<R>
    where
        T: Send,
        F: Fn(T) -> R + Send + Sync,
        R: Send,
    {
        items.into_par_iter().map(f).collect()
    }

    /// Process items in parallel with a chunk size
    pub fn process_parallel_chunked<T, F, R>(
        items: Vec<T>,
        chunk_size: usize,
        f: F,
    ) -> Vec<R>
    where
        T: Send,
        F: Fn(T) -> R + Send + Sync,
        R: Send,
    {
        items
            .into_par_iter()
            .with_min_len(chunk_size)
            .map(f)
            .collect()
    }

    /// Filter items in parallel
    pub fn filter_parallel<T, F>(items: Vec<T>, predicate: F) -> Vec<T>
    where
        T: Send,
        F: Fn(&T) -> bool + Send + Sync,
    {
        items.into_par_iter().filter(predicate).collect()
    }

    /// Reduce items in parallel
    pub fn reduce_parallel<T, F>(items: Vec<T>, identity: T, op: F) -> T
    where
        T: Send + Sync + Clone,
        F: Fn(T, T) -> T + Send + Sync,
    {
        items.into_par_iter().reduce(|| identity.clone(), op)
    }
}

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Some(Duration::from_secs(600)),
            max_lifetime: Some(Duration::from_secs(1800)),
        }
    }
}

/// Batch processing utilities
pub struct BatchProcessor;

impl BatchProcessor {
    /// Process items in batches
    pub async fn process_batches<T, F, Fut, R>(
        items: Vec<T>,
        batch_size: usize,
        f: F,
    ) -> Vec<R>
    where
        T: Clone,
        F: Fn(Vec<T>) -> Fut,
        Fut: std::future::Future<Output = Vec<R>>,
    {
        let mut results = Vec::new();

        for chunk in items.chunks(batch_size) {
            let batch_results = f(chunk.to_vec()).await;
            results.extend(batch_results);
        }

        results
    }

    /// Process items in batches with parallel execution
    pub async fn process_batches_parallel<T, F, Fut, R>(
        items: Vec<T>,
        batch_size: usize,
        f: F,
    ) -> Vec<R>
    where
        T: Send + Clone + 'static,
        F: Fn(Vec<T>) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = Vec<R>> + Send,
        R: Send + 'static,
    {
        let batches: Vec<Vec<T>> = items
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        let mut handles = Vec::new();

        for batch in batches {
            let f_clone = f.clone();
            let handle = tokio::spawn(async move {
                f_clone(batch).await
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(batch_results) = handle.await {
                results.extend(batch_results);
            }
        }

        results
    }
}

