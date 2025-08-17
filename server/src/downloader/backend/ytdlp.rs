#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum YtdlOutput {
    PreProcessing,
    PreDownload {
        video_id: String,
    },
    Downloading {
        eta: Option<f64>,
        downloaded_bytes: f32,
        total_bytes: Option<f32>,
        total_bytes_estimate: Option<f32>,
        elapsed: f32,
        speed: Option<f32>,
        playlist_count: Option<i32>,
        playlist_index: Option<i32>,
    },
    EndOfVideo,
    EndOfPlaylist,
    PostProcessing {
        status: String,
    },
}

#[derive(Debug)]
pub enum YtdlOutputError {
    AlreadyExists,
    PlaylistNotChecked,
    PrivateVideo,
    VideoUnavailable,
    NoPlaylist,
    Other(String),
}

pub fn start(
    threadpool: stp::ThreadPool,
    cfg: crate::downloader::DownloadConfig,
) -> crate::downloader::DownloadHandle {
    use {
        crate::downloader::DownloadHandle,
        nonblock::NonBlockingReader,
        std::{
            process::{Command, Stdio},
            sync::{Arc, atomic::AtomicU32},
        },
    };

    if cfg.url.contains("/playlist?") {
        panic!("Playlists are not supported right now");
    }

    let song_dir = crate::config::songs_path().display().to_string();

    let uuid = uuid::Uuid::new_v4();

    let percentage = Arc::new(AtomicU32::new(0));
    let percentage_clone = percentage.clone();

    let future = threadpool.run(move || {
        let args = [
            "-P",
            &song_dir,
            "-o",
            // "%(title)s.%(ext)s",
            &format!("{uuid}"),
            &cfg.url,
            // To be able to read the output
            "--print",
            r#"before_dl:__{"type": "pre_download", "video_id": "%(id)s"}"#,
            "--print",
            r#"playlist:__{"type": "end_of_playlist"}"#,
            "--print",
            r#"after_video:__{"type": "end_of_video"}"#,
            // Read progress
            "--progress-template",
            concat!(
                r#"__{"type": "downloading","#,
                r#""eta": %(progress.eta)s, "downloaded_bytes": %(progress.downloaded_bytes)s,"#,
                r#""total_bytes": %(progress.total_bytes)s, "total_bytes_estimate": %(progress.total_bytes_estimate)s,"#,
                r#""elapsed": %(progress.elapsed)s, "speed": %(progress.speed)s, "playlist_count": %(info.playlist_count)s,"#,
                r#""playlist_index": %(info.playlist_index)s }"#
            ),
            "--extract-audio",
            "--no-quiet",
            "-x",
            "--audio-format",
            "mp3",
            "--audio-quality",
            "0", // best
            "--write-info-json", // Writes a lot of metadata to {uuid}.info.json
            "--no-cache-dir",
            "--print-traffic"
        ];

        let mut cmd = Command::new("yt-dlp");
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut handle = cmd.spawn().unwrap();

        let mut stdout = NonBlockingReader::from_fd(
            handle.stdout.take().unwrap()
        ).unwrap();
        let mut stderr = NonBlockingReader::from_fd(
            handle.stderr.take().unwrap()
        ).unwrap();
        let mut stdout_buffer = Vec::new();
        let mut stderr_buffer = Vec::new();

        loop {
            let _ = stdout.read_available(&mut stdout_buffer).unwrap();
            if stderr.read_available(&mut stderr_buffer).unwrap() != 0 {
                let s = String::from_utf8_lossy(&stderr_buffer);
                warn!("GOT ERROR: {s}");
                stdout_buffer.extend_from_slice(format!("stderr:{s}").as_bytes());
            }

            let s = String::from_utf8_lossy(&stdout_buffer);
            let progresses = match parse_ytdl(&s){
                Ok(p) => p,
                Err(e) => {
                    error!("Failed to parse progress: {e:?}");
                    break;
                }
            };

            for progress in progresses.into_iter(){
                match progress{
                    YtdlOutput::PreProcessing => {
                        trace!("Pre processing")
                    },
                    YtdlOutput::PreDownload { video_id } => {
                        trace!("Pre download phase with id: {video_id}");
                    },
                    YtdlOutput::Downloading {
                        eta,
                        downloaded_bytes,
                        total_bytes,
                        total_bytes_estimate,
                        speed,
                        ..
                    } => {
                        let prcent = downloaded_bytes / total_bytes.unwrap_or(
                            total_bytes_estimate.unwrap_or(1.)
                        ) * 100.;

                        info!("Downloading\n{prcent}%, Eta: {eta:?}, Speed: {speed:?}");
                        percentage_clone.store(prcent as u32, std::sync::atomic::Ordering::Release);
                    },
                    YtdlOutput::EndOfVideo => {
                        info!("END OF VIDEO")
                    },
                    YtdlOutput::EndOfPlaylist => {
                        info!("END OF PLAYLIST")
                    },
                    YtdlOutput::PostProcessing { status } => {
                        info!("POST PROCESSING: {status}")
                    },
                }
            }
            stdout_buffer.clear();
            stderr_buffer.clear();

            if stdout.is_eof() || stderr.is_eof(){
                println!("EOF");
                break
            };
        }

        handle.wait().unwrap();
        // Here, we know that yt-dlp has exited
        // So we can do some post processing of the downloaded data
        // Since we added the `"--write-info-json"` arg to yt-dlp,
        // we *should* have a {uuid}.info.json file
        // Let's parse that.
        let json_info_path = {
            let mut p = crate::config::songs_path();
            p.push(format!("{uuid}.info.json"));
            p
        };

        debug!("Checking {json_info_path:?}" );

        let json_info_file = match std::fs::OpenOptions::new().read(true).open(&json_info_path){
            Ok(f) => f,
            Err(e) => {
                error!("Could not open upload json info due to: {e}");
                panic!("{e}" )
            },
        };

        let json_info: models::ytdlp_info::YtdlpInfo = match serde_json::from_reader(json_info_file) {
            Ok(ji) => ji,
            Err(e) => {
                error!("Could not deserialiize json info file from download {uuid} due to: {e}");
                panic!("{e}")
            },
        };

        std::fs::remove_file(json_info_path).unwrap();

        debug!("info duration: {}", json_info.duration);

        let metadata = shared::song::Metadata::new(json_info.title, std::time::Duration::from_secs_f64(json_info.duration));

        metadata.write_to_file(uuid, crate::config::songs_path()).unwrap();

        // Aaaand remove the the extension from the output file

        std::fs::rename({
            let mut p = cfg.song_path.clone();
            p.push(format!("{uuid}.mp3"));
            p
        }, {
            let mut p = cfg.song_path.clone();
            p.push(uuid.as_hyphenated().to_string());
            p
        }).unwrap();
    });

    DownloadHandle::new(future, uuid, percentage)
}

pub fn parse_ytdl(output: &str) -> Result<Vec<YtdlOutput>, YtdlOutputError> {
    use serde_json::from_str;

    let output = output.trim();
    if output.is_empty() {
        return Ok(Vec::new());
    }

    if output.contains("has already been downloaded") {
        return Err(YtdlOutputError::AlreadyExists);
    } else if output.contains("entry does not pass filter (!playlist)") {
        return Err(YtdlOutputError::PlaylistNotChecked);
    } else if output.contains("Private video. Sign in if you've been granted access to this video") {
        return Err(YtdlOutputError::PrivateVideo);
    } else if output.contains("Video unavailable. This video contains content") ||
        output.contains("Video unavailable. This video is no longer available because the YouTube account associated with this video has been terminated.") {
        return Err(YtdlOutputError::VideoUnavailable);
    } else if output.contains("YouTube said: The playlist does not exist.") {
        return Err(YtdlOutputError::NoPlaylist);
    } else if let Some(error) = output.strip_prefix("stderr:ERROR: ") {
        return Err(YtdlOutputError::Other(error.to_string()));
    }

    Ok(output
        .lines()
        .filter(|line| line.starts_with("__"))
        .flat_map(|line| {
            line.trim()
                .split("__")
                .map(|s| {
                    if s.trim().contains("send: b'GET") {
                        s.split("send: b'GET").collect::<Vec<&str>>()[0]
                    } else {
                        s
                    }
                })
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .filter_map(|part| {
                    from_str::<_>(&part.replace("NA", "null"))
                        .map_err(|e| error!("{e:?}\n{part}"))
                        .ok()
                })
        })
        .collect::<Vec<_>>())
}
