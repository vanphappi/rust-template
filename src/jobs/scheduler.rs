use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Schedule type
#[derive(Debug, Clone)]
pub enum Schedule {
    Once(DateTime<Utc>),
    Interval(Duration),
    Cron(String),
}

/// Scheduled job
#[derive(Debug, Clone)]
pub struct ScheduledJob {
    pub id: String,
    pub name: String,
    pub schedule: Schedule,
    pub next_run: DateTime<Utc>,
    pub enabled: bool,
}

/// Job scheduler
pub struct JobScheduler {
    jobs: Arc<RwLock<HashMap<String, ScheduledJob>>>,
}

impl JobScheduler {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn schedule(&self, name: String, schedule: Schedule) -> String {
        let job_id = uuid::Uuid::new_v4().to_string();
        let next_run = match &schedule {
            Schedule::Once(dt) => *dt,
            Schedule::Interval(duration) => Utc::now() + *duration,
            Schedule::Cron(_) => Utc::now(), // Placeholder
        };

        let job = ScheduledJob {
            id: job_id.clone(),
            name,
            schedule,
            next_run,
            enabled: true,
        };

        if let Ok(mut jobs) = self.jobs.write() {
            jobs.insert(job_id.clone(), job);
        }

        job_id
    }

    pub fn cancel(&self, job_id: &str) -> bool {
        if let Ok(mut jobs) = self.jobs.write() {
            jobs.remove(job_id).is_some()
        } else {
            false
        }
    }

    pub fn get_scheduled_jobs(&self) -> Vec<ScheduledJob> {
        if let Ok(jobs) = self.jobs.read() {
            jobs.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for JobScheduler {
    fn default() -> Self {
        Self::new()
    }
}

