use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    #[inline]
    pub fn should_cancel(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Clone)]
pub struct Canceller {
    cancelled: Arc<AtomicBool>,
}

impl Canceller {
    #[inline]
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

#[inline]
pub fn cancellation_token() -> (Canceller, CancellationToken) {
    let cancelled = Arc::new(AtomicBool::new(false));
    (
        Canceller {
            cancelled: Arc::clone(&cancelled),
        },
        CancellationToken { cancelled },
    )
}
