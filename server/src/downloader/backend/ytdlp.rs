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

    if cfg.url.contains("/playlist?"){
        panic!("Playlists are not supported right now");
    }

    let song_dir = Box::new(shared::path::songs_path().display().to_string());

    let uuid = uuid::Uuid::new_v4();

    let percentage = Arc::new(AtomicU32::new(0));
    let percentage_clone = percentage.clone();

    let future = threadpool.run(move || {
        let args: [&str; 19] = [
            "-P",
            Box::leak(song_dir), // FIXME
            "-o",
            // "%(title)s.%(ext)s",
            Box::leak(Box::new(format!("{}", uuid::Uuid::new_v4()))), // FIXME
            Box::leak(Box::new(cfg.url)),                             // FIXME
            //
            "--print",
            r#"before_dl:__{"type": "pre_download", "video_id": "%(id)s"}"#,
            "--print",
            r#"playlist:__{"type": "end_of_playlist"}"#,
            "--print",
            r#"after_video:__{"type": "end_of_video"}"#,
            //
            "--progress-template",
            concat!(
                r#"__{"type": "downloading","#,
                r#""eta": %(progress.eta)s, "downloaded_bytes": %(progress.downloaded_bytes)s,"#,
                r#""total_bytes": %(progress.total_bytes)s, "total_bytes_estimate": %(progress.total_bytes_estimate)s,"#,
                r#""elapsed": %(progress.elapsed)s, "speed": %(progress.speed)s, "playlist_count": %(info.playlist_count)s,"#,
                r#""playlist_index": %(info.playlist_index)s }"#
            ),
            "--no-quiet",
            "-x",
            "--audio-format",
            "mp3",
            "--audio-quality",
            "0", // best
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
