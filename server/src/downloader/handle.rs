pub struct DownloadHandle {
    future: stp::ArcFuture<()>,

    uuid: uuid::Uuid,
    url: String,
    phase: std::sync::Arc<std::sync::Mutex<shared::download::Phase>>,
    // metadata: std::sync::Arc<std::sync::Mutex<Option<shared::song::Metadata>>>,
}

impl DownloadHandle {
    pub fn new(
        future: stp::ArcFuture<()>,
        uuid: uuid::Uuid,
        url: String,
        // metadata: std::sync::Arc<std::sync::Mutex<Option<shared::song::Metadata>>>,
        // percentage: std::sync::Arc<std::sync::atomic::AtomicU32>,
        download_phase: std::sync::Arc<std::sync::Mutex<shared::download::Phase>>,
    ) -> Self {
        Self {
            future,
            uuid,
            url,
            phase: download_phase,
            // metadata,
        }
    }

    pub fn uuid(&self) -> &uuid::Uuid {
        &self.uuid
    }
    pub fn url(&self) -> &String {
        &self.url
    }
    // pub fn download_percentage(&self) -> u32 {
    //     use std::sync::atomic::Ordering;
    //     self.current_percentage.load(Ordering::Acquire)
    // }
    pub fn phase(&self) -> shared::download::Phase {
        self.phase.lock().unwrap().clone()
    }

    pub fn state(&self) -> stp::FutureState {
        self.future.state()
    }
}
