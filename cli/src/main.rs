use shared::command::{Command, PlayerCommand};
use shared::message::ClientMessage;
use std::str::FromStr;

#[macro_use]
extern crate log; // trace, debug, info, warn, error

fn main() {
    let cfg = logger::LoggerConfig::default()
        .add_filter("symphonia_core", log::LevelFilter::Off)
        .add_filter("symphonia_bundle_mp3", log::LevelFilter::Off)
        .add_filter("networking", log::LevelFilter::Debug);
    logger::init(cfg, Some("./log/server.log"));

    let mut client = shared::client::Client::new().unwrap();

    client.send(ClientMessage::Ping).unwrap(); // deref works :D

    debug!(
        "Server sent {:?}",
        client.recv(std::time::Duration::from_millis(100))
    );

    debug!("Hello, world!");

    // let song = shared::song::convert_local(
    //     "d:/dev/rust/projects/rmp/songs/840a55ac-1e35-4017-96ff-b997cc12d611".into(), // Not supported
    //     "d:/dev/rust/projects/rmp/songs/".into(),
    // );
    // debug!("Done: {song:?}")

    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::AddToQueue(
                uuid::Uuid::from_str("840a55ac-1e35-4017-96ff-b997cc12d611").unwrap(),
            ),
        )))
        .unwrap();

    debug!(
        "Server sent {:?}",
        client.recv(std::time::Duration::from_millis(100))
    );

    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::SetVolume(0.09),
        )))
        .unwrap();

    debug!(
        "Server sent {:?}",
        client.recv(std::time::Duration::from_millis(100))
    );

    // client
    //     .send(ClientMessage::Command(Command::Player(PlayerCommand::Play)))
    //     .unwrap();

    // debug!(
    //     "Server sent {:?}",
    //     client.recv(std::time::Duration::from_millis(100))
    // );

    #[allow(deprecated)] // temporary
    std::thread::sleep_ms(5000);

    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::Pause,
        )))
        .unwrap();
    debug!(
        "Server sent {:?}",
        client.recv(std::time::Duration::from_millis(100))
    );

    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::ClearQueue,
        )))
        .unwrap();
    debug!(
        "Server sent {:?}",
        client.recv(std::time::Duration::from_millis(100))
    );
}
