#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Downloading, // The data part
    Metadata,
    Done,
    Failed,
}

pub struct DownloadHandle {
    channel: threading::Channel<super::DownloadMessage, ()>,
    thread_handle: std::thread::JoinHandle<()>,

    latest_percentage: f32,
    buffer: Vec<u8>,

    uuid: uuid::Uuid,
    file: std::fs::File,
    metadata: std::sync::Arc<std::sync::Mutex<Option<shared::song::Metadata>>>,

    state: State,
}

impl DownloadHandle {
    pub fn new(
        channel: threading::Channel<super::DownloadMessage, ()>,
        metadata: std::sync::Arc<std::sync::Mutex<Option<shared::song::Metadata>>>,
        thread_handle: std::thread::JoinHandle<()>,
    ) -> Self {
        let uuid = uuid::Uuid::new_v4();

        let file = std::fs::File::create(format!("D:/Dev/Rust/projects/rmp/songs/{uuid}")).unwrap();
        Self {
            channel,
            thread_handle,

            latest_percentage: 0.,
            buffer: Vec::new(),

            uuid,
            file,
            metadata,

            state: State::Downloading,
        }
    }
    pub fn update(&mut self) {
        use super::DownloadMessage;

        debug!("Handle update");

        loop {
            match self.channel.try_recv() {
                Ok(message) => {
                    // debug!("Received a message from downloader: {message:?}");
                    match message {
                        DownloadMessage::PrcentageUpdate(val) => self.latest_percentage = val,
                        DownloadMessage::ChunkUpdate(chunk) => {
                            debug!("Received {}bytes", chunk.len());
                            self.buffer.extend_from_slice(&chunk)
                        }
                        DownloadMessage::Error(e) => {
                            error!("Downloader found an error while, well, downloading,: {e}");
                            warn!("The downloader has failed and exited");

                            self.state = State::Failed;
                            break
                        }
                        DownloadMessage::Done => {
                            self.state = State::Metadata;
                            debug!("The downloader has finished it's job and exited");
                            break;
                        }
                    }
                }
                Err(e) if e == std::sync::mpsc::TryRecvError::Empty => {
                    // This is essentially a would block error, so we break the loop and go to next tick
                    break;
                }
                Err(e) => {
                    error!("Couldn't get a message from the channel due to: {e}");
                    break
                }
            };
        }

        if !self.buffer.is_empty() {
            use std::io::Write as _;
            // ugly for now, we'll need the song dir from config later

            self.file.write_all(&mut self.buffer).unwrap();
            self.buffer.clear();
        }

        if self.state == State::Metadata {
            // TODO: write metadata file
            debug!("now write metadata !");
            let file = std::fs::File::create(format!(
                "D:/Dev/Rust/projects/rmp/songs/{}.metadata",
                self.uuid.as_hyphenated()
            ))
            .unwrap();
            ron::ser::to_writer(file, &self.metadata.lock().unwrap().clone().unwrap()).unwrap();
            self.state = State::Done;
        }
    }

    pub fn download_percentage(&self) -> f32 {
        self.latest_percentage
    }

    pub fn state(&self) -> State {
        self.state
    }
}
