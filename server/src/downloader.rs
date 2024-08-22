use std::{
    fs::metadata,
    io::{Read, Write},
};

use networking::Message;

const CHUNK_SIZE_B: u64 = 1024 * 1024; // 1mb

#[derive(Debug, PartialEq)]
pub enum DownloadMessage {
    PrcentageUpdate(f32),
    ChunkUpdate(Vec<u8>),
    Error(String), // every error means that the downloader has exited
    Done,
}

pub struct DownloadHandle {
    channel: threading::Channel<DownloadMessage, ()>,

    latest_percentage: f32,
    buffer: Vec<u8>,

    uuid: uuid::Uuid,
    file: std::fs::File,
    metadata: std::sync::Arc<std::sync::Mutex<Option<shared::song::Metadata>>>,

    running: bool,
    failed: bool,
}

#[derive(Clone)]
pub struct DownloadConfig {
    pub url: String,
    // ?
}

impl DownloadHandle {
    pub fn new(
        channel: threading::Channel<DownloadMessage, ()>,
        metadata: std::sync::Arc<std::sync::Mutex<Option<shared::song::Metadata>>>,
    ) -> Self {
        let uuid = uuid::Uuid::new_v4();

        let file = std::fs::File::create(format!("D:/Dev/Rust/projects/rmp/songs/{uuid}")).unwrap();
        Self {
            channel,
            latest_percentage: 0.,
            buffer: Vec::new(),

            uuid,
            file,
            metadata,

            running: true,
            failed: false,
        }
    }
    pub fn update(&mut self) {
        match self.channel.try_recv() {
            Ok(message) => {
                // debug!("Received a message from downloader: {message:?}");
                match message {
                    DownloadMessage::PrcentageUpdate(val) => self.latest_percentage = val,
                    DownloadMessage::ChunkUpdate(chunk) => self.buffer.extend_from_slice(&chunk),
                    DownloadMessage::Error(e) => {
                        error!("Downloader found an error while, well, downloading,: {e}");
                        warn!("The downloader has failed and exited");

                        self.running = false;
                        self.failed = true;
                    }
                    DownloadMessage::Done => {
                        self.running = false;
                        self.failed = false;
                        debug!("The downloader has finished it's job and exited")
                    }
                }
            }
            Err(e) => {
                // error!("Couldn't get a message from the channel due to: {e}")
            }
        };

        if !self.buffer.is_empty() {
            use std::io::Write as _;
            // ugly for now, we'll need the song dir from config later

            self.file.write_all(&mut self.buffer).unwrap();
            self.buffer.clear();
        }

        if !self.running && !self.failed {
            // TODO: write metadata file
            debug!("now write metadata !");
            let file = std::fs::File::create(format!(
                "D:/Dev/Rust/projects/rmp/songs/{}.metadata",
                self.uuid.as_hyphenated()
            ))
            .unwrap();
            ron::ser::to_writer(file, &self.metadata.lock().unwrap().clone().unwrap()).unwrap();
        }
    }

    pub fn download_percentage(&self) -> f32 {
        self.latest_percentage
    }

    pub fn running(&self) -> bool {
        self.running
    }
}

// Downloads a given url and import the downloaded data as a new song
pub fn download(config: DownloadConfig) -> DownloadHandle {
    let DownloadConfig { url } = config;

    let (channel1, channel2) = threading::Channel::new_pair();

    let video_options = rusty_ytdl::VideoOptions {
        quality: rusty_ytdl::VideoQuality::HighestAudio,
        filter: rusty_ytdl::VideoSearchOptions::Audio,
        download_options: rusty_ytdl::DownloadOptions {
            dl_chunk_size: Some(CHUNK_SIZE_B), // 1MB / packet
        },
        ..Default::default()
    };

    let metadata_arc = std::sync::Arc::new(std::sync::Mutex::new(None));
    let cloned = metadata_arc.clone();

    debug!("Creating thread");

    let handle = std::thread::Builder::new()
        .name(format!("Downloader thread for id: {url}"))
        // .stack_size((CHUNK_SIZE_B as usize * 2) as usize)
        .spawn(move || {
            debug!("Eh ?");
            debug!("Runtime created");
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                // futures_lite::future::block_on(async {
                // debug!("Starting thread");
                println!("Thread");
                let Ok(video) = rusty_ytdl::Video::new_with_options(&url, video_options) else {
                    channel2
                        .send(DownloadMessage::Error(
                            "Could not create ytdl video".to_owned(),
                        ))
                        .unwrap();
                    return;
                };

                let Ok(stream) = video
                    .stream_with_ffmpeg(Some(rusty_ytdl::FFmpegArgs {
                        format: Some("mp3".to_string()),
                        audio_filter: Some("aresample=48000,asetrate=48000*0.8".to_string()),
                        video_filter: None,
                    }))
                    .await
                else {
                    channel2
                        .send(DownloadMessage::Error(
                            "Could not build video's stream".to_owned(),
                        ))
                        .unwrap();
                    return;
                };
                let info = video.get_basic_info().await.unwrap().video_details;

                *cloned.lock().unwrap() = Some(shared::song::Metadata::new(
                    info.title,
                    std::time::Duration::from_secs(info.length_seconds.parse().unwrap()),
                ));

                debug!("Starting download");
                loop {
                    match stream.chunk().await {
                        Ok(bytes_opt) => {
                            let Some(bytes) = bytes_opt else {
                                debug!("Download's done");
                                break;
                            };

                            // Do something with the chunk

                            // if bytes.len() as u64 > CHUNK_SIZE_B {
                            //     channel2
                            //     .send(DownloadMessage::Error(format!(
                            //         "An error occured while downloading video ({url}): Received a chunk too large for transfer: {}b", bytes.len()
                            //     )))
                            //     .unwrap();
                            //     return;
                            // }

                            // let mut buffer = [0u8; CHUNK_SIZE_B as usize];

                            // let mut x = bytes.take(CHUNK_SIZE_B);
                            // if let Err(e) = x.read(&mut buffer){
                            //     channel2.send(
                            //         DownloadMessage::Error(format!("Failed to take {CHUNK_SIZE_B}bytes from stream due to: {e}"))
                            //     ).unwrap();

                            //     return;
                            // }

                            channel2
                                .send(DownloadMessage::ChunkUpdate(bytes.to_vec()))
                                .unwrap();
                            // debug!("sent chunk update");
                        }
                        Err(e) => {
                            channel2
                                .send(DownloadMessage::Error(format!(
                                    "An error occured while downloading video ({url}): {e}"
                                )))
                                .unwrap();
                            return;
                        }
                    }
                }
                channel2.send(DownloadMessage::Done).unwrap();
                warn!("Exiting download thread")
            })
        });

    DownloadHandle::new(channel1, metadata_arc)
}
