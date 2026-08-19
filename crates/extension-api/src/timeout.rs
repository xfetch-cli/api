//! Timeout helpers for extensions.
//!
//! Extensions have variable runtimes (config providers may hit the network
//! or do heavy work), so each extension declares its own budget in its code.
//! `with_timeout` runs the task on a worker thread and returns `Err(TimedOut)`
//! when the budget elapses; the extension can then respond with a fallback
//! config and exit normally, so the core's `wait_with_output` always returns.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// The task did not finish within its budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimedOut;

/// Runs `task` on a worker thread, waiting at most `budget`.
///
/// Returns `Ok(task())` when the task finishes in time, `Err(TimedOut)`
/// otherwise. The worker thread keeps running until the process exits.
pub fn with_timeout<T: Send + 'static>(
    budget: Duration,
    task: impl FnOnce() -> T + Send + 'static,
) -> Result<T, TimedOut> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(task());
    });
    rx.recv_timeout(budget).map_err(|_| TimedOut)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_completing_within_budget_returns_value() {
        let result = with_timeout(Duration::from_secs(5), || 42u32);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn slow_task_times_out() {
        let started = std::time::Instant::now();
        let result = with_timeout(Duration::from_millis(50), || {
            std::thread::sleep(Duration::from_secs(5));
            1u32
        });
        assert_eq!(result, Err(TimedOut));
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "timeout must fire promptly, took {:?}",
            started.elapsed()
        );
    }
}
