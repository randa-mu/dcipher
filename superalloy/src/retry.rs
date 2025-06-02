use std::fmt::Debug;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum RetryStrategy {
    None,
    ConstantBackoff { retries: usize, backoff: Duration },
}

//noinspection RsCompilerFeatureIsUnavailable,RsCallExpr
#[tracing::instrument(skip(fn_to_retry))]
pub async fn with_retry<T, E, Fn>(fn_to_retry: Fn, strategy: RetryStrategy) -> Result<T, E>
where
    E: Debug,
    Fn: AsyncFn() -> Result<T, E>,
{
    // Always execute at least once
    let mut last_err = match fn_to_retry().await {
        Ok(t) => return Ok(t),
        Err(e) => e,
    };

    match strategy {
        RetryStrategy::None => Err(last_err),

        RetryStrategy::ConstantBackoff { retries, backoff } => {
            // Do retries
            for current_try in 0..retries {
                tracing::error!(current_try, error=?last_err, "failed to execute function");
                tokio::time::sleep(backoff).await;

                last_err = match fn_to_retry().await {
                    Ok(t) => return Ok(t),
                    Err(e) => e,
                }
            }

            Err(last_err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[tokio::test]
    async fn test_constant_backoff() {
        let retries = 10;
        let strategy = RetryStrategy::ConstantBackoff {
            retries,
            backoff: Duration::from_millis(0),
        };

        // Use an atomic integer to satisfy the AsyncFn bound (instead of AsyncFnMut) through interior mutability
        let mut success_fn_counter = AtomicUsize::new(0);
        let success_fn = async || {
            success_fn_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        };
        // On success, it should only execute it once
        let res: Result<(), std::io::Error> = with_retry(success_fn, strategy).await;
        assert!(res.is_ok());
        assert_eq!(success_fn_counter.get_mut().to_owned(), 1);

        // Use an atomic integer to satisfy the AsyncFn bound (instead of AsyncFnMut) through interior mutability
        let mut failure_fn_counter = AtomicUsize::new(0);
        let failure_fn = async || {
            failure_fn_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Err(std::io::Error::other("failure"))
        };
        // On failures, it should only execute it retries + 1 times
        let res: Result<(), std::io::Error> = with_retry(failure_fn, strategy).await;
        assert!(res.is_err());
        assert_eq!(failure_fn_counter.get_mut().to_owned(), retries + 1);
    }
}
