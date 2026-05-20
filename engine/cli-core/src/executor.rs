//! B4n1Web Task Executor - Parallel execution with configurable limits

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    pub max_concurrent: usize,
    pub retry_limit: u32,
    pub retry_delay_ms: u64,
    pub request_timeout_ms: u64,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self { max_concurrent: 5, retry_limit: 3, retry_delay_ms: 1000, request_timeout_ms: 30000 }
    }
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub value: String,
    pub attempts: u32,
    pub duration_ms: u64,
}

pub struct Executor {
    config: ExecutorConfig,
    semaphore: Arc<Semaphore>,
    active_count: Arc<AtomicUsize>,
    total_completed: Arc<AtomicUsize>,
    total_errors: Arc<AtomicUsize>,
}

impl Executor {
    pub fn new() -> Self { Self::with_config(ExecutorConfig::default()) }

    pub fn with_config(config: ExecutorConfig) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(config.max_concurrent)),
            active_count: Arc::new(AtomicUsize::new(0)),
            total_completed: Arc::new(AtomicUsize::new(0)),
            total_errors: Arc::new(AtomicUsize::new(0)),
            config,
        }
    }

    pub fn config(&self) -> &ExecutorConfig { &self.config }
    pub fn active_count(&self) -> usize { self.active_count.load(Ordering::Relaxed) }
    pub fn completed_count(&self) -> usize { self.total_completed.load(Ordering::Relaxed) }
    pub fn error_count(&self) -> usize { self.total_errors.load(Ordering::Relaxed) }

    /// Execute an async task with retry and timeout
    pub async fn execute<F, T>(&self, task: F) -> Result<TaskResult, String>
    where
        F: Fn() -> T,
        T: std::future::Future<Output = Result<String, String>>,
    {
        let _permit = self.semaphore.acquire().await.map_err(|e| format!("Sem: {}", e))?;
        self.active_count.fetch_add(1, Ordering::Relaxed);
        let start = std::time::Instant::now();
        let mut last_err = String::new();

        for attempt in 1..=self.config.retry_limit {
            let timeout = Duration::from_millis(self.config.request_timeout_ms);
            match tokio::time::timeout(timeout, task()).await {
                Ok(Ok(val)) => {
                    let dur = start.elapsed().as_millis() as u64;
                    self.active_count.fetch_sub(1, Ordering::Relaxed);
                    self.total_completed.fetch_add(1, Ordering::Relaxed);
                    return Ok(TaskResult { value: val, attempts: attempt, duration_ms: dur });
                }
                Ok(Err(e)) => last_err = e,
                Err(_) => last_err = "timeout".into(),
            }
            if attempt < self.config.retry_limit {
                sleep(Duration::from_millis(self.config.retry_delay_ms)).await;
            }
        }

        self.active_count.fetch_sub(1, Ordering::Relaxed);
        self.total_errors.fetch_add(1, Ordering::Relaxed);
        Err(format!("Failed after {} retries: {}", self.config.retry_limit, last_err))
    }
}

impl Default for Executor {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_default_config() {
        assert_eq!(Executor::new().config().max_concurrent, 5);
    }

    #[tokio::test]
    async fn test_execute_ok() {
        let ex = Executor::new();
        let r = ex.execute(|| async { Ok::<_, String>("hi".into()) }).await;
        assert_eq!(r.unwrap().value, "hi");
    }

    #[tokio::test]
    async fn test_execute_retry() {
        let c = Arc::new(AtomicUsize::new(0));
        let cc = c.clone();
        let ex = Executor::new();
        let r = ex.execute(move || {
            let c = cc.clone();
            async move {
                if c.fetch_add(1, Ordering::SeqCst) == 0 { Err::<_, String>("fail".into()) }
                else { Ok::<_, String>("ok".into()) }
            }
        }).await;
        assert!(r.unwrap().attempts > 1);
    }

    #[tokio::test]
    async fn test_execute_timeout() {
        let cfg = ExecutorConfig { max_concurrent: 1, retry_limit: 1, retry_delay_ms: 0, request_timeout_ms: 10 };
        let ex = Executor::with_config(cfg);
        let r = ex.execute(|| async {
            sleep(Duration::from_millis(100)).await;
            Ok::<_, String>("x".into())
        }).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn test_counts() {
        let ex = Executor::new();
        ex.execute(|| async { Ok::<_, String>("a".into()) }).await.unwrap();
        assert_eq!(ex.completed_count(), 1);
    }
}
