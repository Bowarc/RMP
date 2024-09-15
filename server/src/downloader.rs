mod config;
mod handle;
mod message;

pub use config::DownloadConfig;
use handle::DownloadHandle;
use message::DownloadMessage;

const CHUNK_SIZE_B: u64 = 1024 * 1024; // 1mb

#[derive(Default)]
pub struct DownloadManager /* hate naming things like that but eh */ {
    queue: std::collections::VecDeque<DownloadConfig>,

    current: Option<DownloadHandle>,
}

impl DownloadManager {
    pub fn push(&mut self, cfg: DownloadConfig) -> () {
        self.queue.push_back(cfg)
    }

    pub fn update(&mut self) -> Result<(), shared::server::error::DownloaderError>{
        if let Some(current) = &mut self.current{
            debug!("Updating current ({:?})", current.state());
            current.update();
            if current.state() == handle::State::Done{
                debug!("The current download has finished");
                self.current = None;
            }
        }
        // could use else if but i don't like implicit `self.current.is_none()`
        if self.current.is_none() && !self.queue.is_empty() {
            let new = self.queue.pop_front().unwrap();
            debug!("Current got free'd, creating new one with url: {}", new.url);
            self.current = Some(download(new));
        }

        Ok(())
    }

    pub fn current(&self) -> Option<&DownloadHandle>{
        self.current.as_ref()
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

                // This works but
                // It creates a lot more samples,
                // Cannot use static array types as samples don't have known size
                // Requires ffmpeg
                //

                // The conversion fucks up the % calc
                // Is it ? koz it also fucked with the music duration, sooo it might just be the ffmpeg audio filter !

                let Ok(stream) = video
                    .stream_with_ffmpeg(Some(rusty_ytdl::FFmpegArgs {
                        format: Some("mp3".to_string()),
                        audio_filter: None,
                        video_filter: None,
                    }))
                    .await
                // let Ok(stream) = video.stream().await
                else {
                    channel2
                        .send(DownloadMessage::Error(
                            "Could not build video's stream".to_owned(),
                        ))
                        .unwrap();
                    return;
                };
                let info = video.get_basic_info().await.unwrap().video_details;

                debug!(
                    "The video is {} seconds long",
                    info.length_seconds.parse::<u64>().unwrap()
                );

                *cloned.lock().unwrap() = Some(shared::song::Metadata::new(
                    info.title,
                    std::time::Duration::from_secs(info.length_seconds.parse().unwrap()),
                ));

                debug!("Starting download");

                let total_download_size = stream.content_length();
                let mut downloaded = 0;
                loop {
                    match stream.chunk().await {
                        Ok(bytes_opt) => {
                            let Some(bytes) = bytes_opt else {
                                debug!("Download's done");
                                break;
                            };
                            downloaded += bytes.len();

                            channel2
                                .send(DownloadMessage::PrcentageUpdate(
                                    // TODO: Fix multiplier here, find conversion rate for mp3 from m4a
                                    (downloaded as f32 / (total_download_size as f32)) * 100.,
                                ))
                                .unwrap();

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
        })
        .unwrap();

    DownloadHandle::new(channel1, metadata_arc, handle)
}
