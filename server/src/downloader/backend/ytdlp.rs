pub fn start(
    threadpool: stp::ThreadPool,
    cfg: crate::downloader::DownloadConfig,
) -> crate::downloader::DownloadHandle {
    use crate::downloader::DownloadHandle;
    use std::sync::atomic::AtomicU32;
    use std::{
        process::{Command, Stdio},
        sync::Arc,
    };
    let song_dir = Box::new(shared::path::songs_path().display().to_string());

    let args = [
        "-P",
        Box::leak(song_dir), // FIXME
        "-o",
        "%(title)s.%(ext)s",
        Box::leak(Box::new(cfg.url)), // FIXME
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
        "0" // best

        
    ];

    let uuid = uuid::Uuid::new_v4();

    let percentage = Arc::new(AtomicU32::new(0));
    let percentage_clone = percentage.clone();

    let tpc = threadpool.clone();

    let future = threadpool.run(move || {
        let percentage = percentage_clone;

        let mut cmd = Command::new("yt-dlp");
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut handle = cmd.spawn().unwrap();

        let stdout = handle.stdout.take().unwrap();
        let _stdout_task = tpc.run(move || {
            use std::io::BufRead as _;
            let mut reader = std::io::BufReader::new(stdout);
            let mut buffer = Vec::new();

            loop {
                debug!("Loop out");
                let read = reader.read_until(b'\r', &mut buffer).unwrap();
                if read == 0 {
                    break;
                }
                debug!("Ytdlp out: {:?}", String::from_utf8_lossy(&buffer));
                buffer.clear();
            }
        });

        let stderr = handle.stderr.take().unwrap();
        let _stderr_task = tpc.run(move || {
            use std::io::BufRead as _;
            let mut reader = std::io::BufReader::new(stderr);
            let mut buffer = Vec::new();

            loop {
                let read = reader.read_until(b'\r', &mut buffer).unwrap();
                if read == 0 {
                    break;
                }
                debug!("Ytdlp err: {:?}", String::from_utf8_lossy(&buffer));
                buffer.clear();
            }
        });
        handle.wait().unwrap();
    });

    DownloadHandle::new(future, uuid, percentage)
}
