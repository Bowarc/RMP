#[macro_use]
extern crate log;

mod config;
mod downloader;
mod player;
mod playlist_manager;
mod server;

use shared::server::error;

const TARGET_TPS: f32 = 10.;

fn main() {
    let cfg = logger::LoggerConfig::default()
        .add_filter("symphonia_core", log::LevelFilter::Off)
        .add_filter("symphonia_bundle_mp3", log::LevelFilter::Off)
        .add_filter("networking", log::LevelFilter::Debug);
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

    //

    let mut music_player: Box<dyn player::Player> =
        Box::new(player::PlayerImpl::new(Default::default()).unwrap());

    let mut socket_opt: Option<
        networking::proxy::ProxyController<shared::message::ClientMessage, shared::message::ServerMessage>,
    > = None;

    let listener = std::net::TcpListener::bind(shared::DEFAULT_ADDRESS).unwrap();
    listener.set_nonblocking(true).unwrap();

    debug!("Starting loop with {TARGET_TPS} TPS");
    while running.load(std::sync::atomic::Ordering::Acquire) {
        // ignore any connection if there is a client already connected
        if socket_opt.is_none() {
            if let Ok((stream, addr)) = listener.accept() {
                info!("New connection from {addr}");
                socket_opt = Some(networking::Proxy::start_new(
                    config::default_proxy_config(),
                    Some(stream),
                ))
            }
        }
        if let Err(e) = music_player.update() {
            error!("An error occured while updating the music player: {e}");
            if let Some(socket) = &mut socket_opt {
                socket
                    .send(shared::message::ServerMessage::Error(
                        shared::server::error::Error::PlayerError(e),
                    ))
                    .unwrap();
            }
        };

        server::recv_commands(&mut socket_opt, &mut music_player);

        interval.tick();
    }

    debug!(
        "Stopping loop. The server ran {}",
        time::format(stopwatch.read(), 3)
    );
}
