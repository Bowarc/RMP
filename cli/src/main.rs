mod cmd_parser;
mod commands;

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

    match cmd_parser::cmd().get_matches().subcommand() {
        Some(("play", _args)) => {
            debug!("Play command");
            unimplemented!()
        }
        Some(("pause", _args)) => {
            debug!("Pause command");
            unimplemented!()
        }
        Some(("queue", args)) => {
            debug!("Queue");

            match args.subcommand() {
                Some(("add", args)) => {
                    debug!("Add");
                    let arg = args.get_one::<String>("ARG").unwrap();

                    if let Ok(uuid) = uuid::Uuid::from_str(arg) {
                        debug!("uuid: {uuid:?}");
                    } else {
                        debug!("name: {arg:?}");
                    }
                    unimplemented!()
                }
                Some(("remove", args)) => {
                    debug!("Remove");
                    let arg = args.get_one::<String>("ARG").unwrap();

                    if let Ok(uuid) = uuid::Uuid::from_str(arg) {
                        debug!("uuid: {uuid:?}");
                    } else {
                        debug!("name: {arg:?}");
                    }
                    unimplemented!()
                }
                Some(("clear", args)) => {
                    debug!("Clear");
                    unimplemented!()
                }
                Some(unhandled) => {
                    error!("Unexpected queue subcommand: {}", unhandled.0)
                }
                None => todo!(),
            }
        }
        Some(("volume", args)) => {
            debug!("Volume");
            match args.subcommand() {
                Some(("get", _args)) => {
                    debug!("get");
                    unimplemented!()
                }
                Some(("set", args)) => {
                    debug!("Remove");
                    let amount = args.get_one::<f64>("AMNT").unwrap();
                    debug!("{amount}");
                    unimplemented!()
                }
                Some(("up", args)) => {
                    debug!("Up");
                    let amount = args.get_one::<f64>("AMNT").unwrap();
                    debug!("{amount}");
                    unimplemented!()
                }
                Some(("down", args)) => {
                    debug!("Down");
                    let amount = args.get_one::<f64>("AMNT").unwrap();
                    debug!("{amount}");
                    unimplemented!()
                }
                Some(unhandled) => {
                    error!("Unexpected volume subcommand: {}", unhandled.0);
                    unimplemented!()
                }
                None => todo!(),
            }
        }
        Some(("device", args)) => {
            debug!("Device");
            match args.subcommand() {
                Some(("get", _args)) => {
                    debug!("Get");
                    unimplemented!()
                }
                Some(("set", args)) => {
                    debug!("Set");
                    let device_name = args.get_one::<String>("NAME").unwrap();
                    debug!("{device_name}");
                    unimplemented!()
                }
                Some(unhandled) => {
                    error!("Unexpected volume subcommand: {}", unhandled.0);
                    unimplemented!()
                }
                None => todo!(),
            }
        }
        Some(unhandled) => {
            debug!("Unhandled {unhandled:?}")
        }
        None => todo!(),
    }
}
