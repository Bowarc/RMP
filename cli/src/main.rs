mod cmd_parser;
mod commands;

use shared::command::{Command, PlayerCommand};
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

    match cmd_parser::cmd().get_matches().subcommand() {
        Some(("play", _args)) => commands::player::play(&mut client),
        Some(("pause", _args)) => commands::player::pause(&mut client),
        Some(("queue", args)) => {

            match args.subcommand() {
                Some(("add", args)) => {
                    let arg = args.get_one::<String>("ARG").unwrap();

                    let si = if let Ok(uuid) = uuid::Uuid::from_str(arg) {
                        commands::player::queue::SongIdentifier::Uuid(uuid)
                    } else {
                        commands::player::queue::SongIdentifier::Title(arg.to_owned())
                    };

                    commands::player::queue::add(&mut client, si);
                }
                Some(("remove", args)) => {
                    let arg = args.get_one::<String>("ARG").unwrap();

                    let si = if let Ok(uuid) = uuid::Uuid::from_str(arg) {
                        commands::player::queue::SongIdentifier::Uuid(uuid)
                    } else {
                        commands::player::queue::SongIdentifier::Title(arg.to_owned())
                    };

                    commands::player::queue::remove(&mut client, si);
                }
                Some(("clear", _args)) => {
                    commands::player::queue::clear(&mut client)
                }
                _ => unreachable!()
            }
        }
        Some(("volume", args)) => {
            match args.subcommand() {
                Some(("get", _args)) => {
                    let out = commands::player::volume::get(&mut client);

                    debug!("{out}")
                }
                Some(("set", args)) => {
                    let amount = args.get_one::<f32>("AMNT").unwrap();
                    commands::player::volume::set(&mut client,*amount);
                }
                Some(("up", args)) => {
                    let amount = args.get_one::<f32>("AMNT").unwrap();

                    commands::player::volume::up(&mut client, *amount)
                }
                Some(("down", args)) => {
                    let amount = args.get_one::<f32>("AMNT").unwrap();

                    commands::player::volume::down(&mut client, *amount)
                }
                _ => unreachable!()
            }
        }
        Some(("device", args)) => {
            match args.subcommand() {
                Some(("get", _args)) => {
                    let name = commands::player::device::get(&mut client);

                    debug!("{name}")
                }
                Some(("set", args)) => {
                    let device_name = args.get_one::<String>("NAME").unwrap();
                    commands::player::device::set(&mut client, device_name.to_owned())
                }
                 _ => unreachable!()
            }
        }
        _ => unreachable!()

        // Abouuut downloader ? :D
    }
}
