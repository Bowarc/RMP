mod backend;
mod config;
mod handle;

pub use config::DownloadConfig;
use handle::DownloadHandle;

// Multiple download backend ?
pub struct DownloadManager /* hate naming things like that but eh */ {
    queue: std::collections::VecDeque<DownloadConfig>,

    threadpool: stp::ThreadPool,

    current: Option<DownloadHandle>,

    old: Vec<DownloadHandle>,
}

impl DownloadManager {
    pub fn push(&mut self, cfg: DownloadConfig) {
        self.queue.push_back(cfg)
    }

    pub fn update(&mut self) -> Result<(), shared::error::server::DownloaderError> {
        if let Some(current) = &mut self.current {
            // debug!("Updating current ({:?}) ({}%)", current.state(), current.download_percentage());
            match current.state() {
                stp::FutureState::Done => {
                    debug!("The current download has finished");
                    self.old.push(self.current.take().unwrap());
                }
                stp::FutureState::Panicked => {
                    warn!("Current download failed.");
                    self.old.push(self.current.take().unwrap());
                }
                _ => (),
            }
        }

        if self.current.is_none()
            && let Some(new) = self.queue.pop_front()
        {
            debug!("Current got free'd, creating new one with url: {}", new.url);

            self.current = Some(backend::ytdlp::start(self.threadpool.clone(), new));
        }

        Ok(())
    }

    pub fn current(&self) -> Option<&DownloadHandle> {
        self.current.as_ref()
    }

    pub fn report_currents(&self) -> Vec<shared::download::Report> {
        let mut out = Vec::new();

        if let Some(handle) = &self.current {
            out.push(shared::download::Report {
                uuid: *handle.uuid(),
                url: handle.url().clone(),
                phase: handle.phase(),
            });
        }

        for handle in self.old.iter() {
            out.push(shared::download::Report {
                uuid: *handle.uuid(),
                url: handle.url().clone(),
                phase: handle.phase(),
            })
        }

        out
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        use {std::collections::VecDeque, stp::ThreadPool};

        Self {
            queue: VecDeque::new(),
            threadpool: ThreadPool::new(1), // Im not sure having threads up 24/7 is a good usage of resources lmao
            current: None,
            old: Vec::new(),
        }
    }
}
