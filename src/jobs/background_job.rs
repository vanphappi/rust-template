use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::errors::ApiError;

/// Job status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Retrying,
}

/// Job result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
}

/// Job trait
#[async_trait]
pub trait Job: Send + Sync + 'static {
    async fn execute(&self) -> Result<JobResult, ApiError>;
    fn job_type(&self) -> &str;
    fn max_retries(&self) -> u32 {
        3
    }
}

/// Job metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobMetadata {
    pub id: String,
    pub job_type: String,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub result: Option<JobResult>,
}

/// Job executor
pub struct JobExecutor {
    jobs: Arc<RwLock<HashMap<String, JobMetadata>>>,
}

impl JobExecutor {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn submit<J: Job>(&self, job: J) -> Result<String, ApiError> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let metadata = JobMetadata {
            id: job_id.clone(),
            job_type: job.job_type().to_string(),
            status: JobStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            retry_count: 0,
            max_retries: job.max_retries(),
            result: None,
        };

        if let Ok(mut jobs) = self.jobs.write() {
            jobs.insert(job_id.clone(), metadata);
        }

        // Spawn background task
        let jobs_clone = self.jobs.clone();
        let job_id_clone = job_id.clone();
        tokio::spawn(async move {
            Self::execute_job(jobs_clone, job_id_clone, job).await;
        });

        Ok(job_id)
    }

    async fn execute_job<J: Job>(
        jobs: Arc<RwLock<HashMap<String, JobMetadata>>>,
        job_id: String,
        job: J,
    ) {
        // Update status to running
        if let Ok(mut jobs_map) = jobs.write() {
            if let Some(metadata) = jobs_map.get_mut(&job_id) {
                metadata.status = JobStatus::Running;
                metadata.started_at = Some(Utc::now());
            }
        }

        // Execute job
        let result = job.execute().await;

        // Update status based on result
        if let Ok(mut jobs_map) = jobs.write() {
            if let Some(metadata) = jobs_map.get_mut(&job_id) {
                match result {
                    Ok(job_result) => {
                        metadata.status = JobStatus::Completed;
                        metadata.completed_at = Some(Utc::now());
                        metadata.result = Some(job_result);
                    }
                    Err(_) => {
                        if metadata.retry_count < metadata.max_retries {
                            metadata.status = JobStatus::Retrying;
                            metadata.retry_count += 1;
                        } else {
                            metadata.status = JobStatus::Failed;
                            metadata.completed_at = Some(Utc::now());
                        }
                    }
                }
            }
        }
    }

    pub fn get_job_status(&self, job_id: &str) -> Option<JobMetadata> {
        if let Ok(jobs) = self.jobs.read() {
            jobs.get(job_id).cloned()
        } else {
            None
        }
    }
}

impl Default for JobExecutor {
    fn default() -> Self {
        Self::new()
    }
}

