#[macro_use]
extern crate log;

mod config;
mod downloader;
mod error;
mod player;
mod playlist_manager;

const TARGET_TPS: f32 = 10.;

fn main() {
    let cfg = logger::LoggerConfig::default();

    logger::init(cfg, Some("./log/server.log"));

    let stopwatch = time::Stopwatch::start_new();

    trace!(
        "\n╭{line}╮\n│{message:^30}│\n╰{line}╯",
        line = "─".repeat(30),
        message = "Server start"
    );

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut interval =
        spin_sleep_util::interval(std::time::Duration::from_secs_f32(1. / TARGET_TPS));

    debug!("Starting loop with {TARGET_TPS}TPS");
    while running.load(std::sync::atomic::Ordering::Acquire) {

        interval.tick();
    };

    debug!(
        "Stopping loop. The server ran {}",
        time::format(stopwatch.read(), 3)
    );
}
