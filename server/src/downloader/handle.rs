
pub struct DownloadHandle {
    future: stp::ArcFuture<()>,

    current_percentage: std::sync::Arc<std::sync::atomic::AtomicU32>,

    uuid: uuid::Uuid,
    // metadata: std::sync::Arc<std::sync::Mutex<Option<shared::song::Metadata>>>,
}

impl DownloadHandle {
    pub fn new(
        future: stp::ArcFuture<()>,
        uuid: uuid::Uuid,
        // metadata: std::sync::Arc<std::sync::Mutex<Option<shared::song::Metadata>>>,
        percentage: std::sync::Arc<std::sync::atomic::AtomicU32>,
    ) -> Self {
        
        Self {
            future,
            current_percentage: percentage,
            uuid,
            // metadata,
        }
    }

    pub fn download_percentage(&self) -> u32 {
        use std::sync::atomic::Ordering;
        self.current_percentage.load(Ordering::Acquire)
    }

    pub fn state(&self) -> stp::FutureState {
        self.future.state()
    }
}
