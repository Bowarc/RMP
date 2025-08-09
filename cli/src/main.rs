use std::str::FromStr;

mod cmd_parser;
mod commands;

#[macro_use]
extern crate log; // trace, debug, info, warn, error

fn main() {
    let filters = &[
        ("symphonia_core", log::LevelFilter::Off),
        ("symphonia_bundle_mp3", log::LevelFilter::Off),
        ("networking", log::LevelFilter::Debug),
    ];
    logger::init([
        logger::Config::default()
            .filters(filters)
            .colored(true)
            .output(logger::Output::Stdout),
        logger::Config::default()
            .filters(filters)
            .output(logger::Output::File(
                std::path::PathBuf::from_str("./log/cli.log").unwrap(),
            )),
    ]);

    let mut socket = shared::socket::new().unwrap();

    match cmd_parser::cmd().get_matches().subcommand() {
        Some(("player", args)) => match args.subcommand() {
            Some(("play", _args)) => commands::player::play(&mut socket),
            Some(("pause", _args)) => commands::player::pause(&mut socket),
            Some(("now_playing", _args)) => {
                let (song, index) = commands::player::now_playing(&mut socket);

                debug!(
                    "Currently playing ({index}) {title}",
                    title = song.metadata().title()
                );
            }
            Some(("queue", args)) => match args.subcommand() {
                Some(("get", _args)) => {
                    let queue = commands::player::queue::get(&mut socket);

                    debug!(
                        "{:?}",
                        queue
                            .iter()
                            .map(|s| s.metadata().title().clone())
                            .collect::<Vec<_>>()
                    );
                }
                Some(("add", args)) => {
                    use std::str::FromStr as _;
                    let arg = args.get_one::<String>("ARG").unwrap();
                    let si = if let Ok(uuid) = uuid::Uuid::from_str(arg) {
                        commands::player::queue::SongIdentifier::Uuid(uuid)
                    } else {
                        commands::player::queue::SongIdentifier::Title(arg.to_owned())
                    };
                    commands::player::queue::add(&mut socket, si);
                }
                Some(("remove", args)) => {
                    use std::str::FromStr as _;
                    let arg = args.get_one::<String>("ARG").unwrap();
                    let si = if let Ok(uuid) = uuid::Uuid::from_str(arg) {
                        commands::player::queue::SongIdentifier::Uuid(uuid)
                    } else {
                        commands::player::queue::SongIdentifier::Title(arg.to_owned())
                    };
                    commands::player::queue::remove(&mut socket, si);
                }
                Some(("clear", _args)) => commands::player::queue::clear(&mut socket),
                _ => unreachable!(),
            },
            Some(("volume", args)) => match args.subcommand() {
                Some(("get", _args)) => {
                    let out = commands::player::volume::get(&mut socket);
                    debug!("{out}")
                }
                Some(("set", args)) => {
                    let amount = args.get_one::<f32>("VALUE").unwrap();
                    commands::player::volume::set(&mut socket, *amount);
                }
                Some(("up", args)) => {
                    let amount = args.get_one::<f32>("AMNT").unwrap();
                    commands::player::volume::up(&mut socket, *amount)
                }
                Some(("down", args)) => {
                    let amount = args.get_one::<f32>("AMNT").unwrap();
                    commands::player::volume::down(&mut socket, *amount)
                }
                _ => unreachable!(),
            },
            Some(("position", args)) => match args.subcommand() {
                Some(("get", _args)) => {
                    let pos = commands::player::position::get(&mut socket);
                    debug!("Player's currently at {pos:?}");
                }
                Some(("set", args)) => {
                    let position_s = args.get_one::<u64>("VALUE").unwrap();

                    commands::player::position::set(
                        &mut socket,
                        std::time::Duration::from_secs(*position_s),
                    )
                }
                Some(("forward", args)) => {
                    let position_s = args.get_one::<u64>("AMNT").unwrap();

                    commands::player::position::forward(
                        &mut socket,
                        std::time::Duration::from_secs(*position_s),
                    )
                }
                Some(("backward", args)) => {
                    let position_s = args.get_one::<u64>("AMNT").unwrap();

                    commands::player::position::backward(
                        &mut socket,
                        std::time::Duration::from_secs(*position_s),
                    )
                }
                _ => unreachable!(),
            },
            Some(("device", args)) => match args.subcommand() {
                Some(("get", _args)) => {
                    let name = commands::player::device::get(&mut socket);
                    debug!("{name}")
                }
                Some(("set", args)) => {
                    let device_name = args.get_one::<String>("NAME").unwrap();
                    commands::player::device::set(&mut socket, device_name.to_owned())
                }
                _ => unreachable!(),
            },

            _ => unreachable!(),
        },
        Some(("downloader", args)) => {
            match args.subcommand() {
                Some(("start", args)) => {
                    let url = args.get_one::<String>("URL").unwrap(); // TODO: Take care of this one
                    commands::downloader::download(&mut socket, url.to_owned());
                }
                _ => unimplemented!(),
            }
        }
        Some(("library", args)) => match args.subcommand() {
            Some(("get", _args)) => {
                let songs = commands::get_library(&mut socket);
                println!(
                    "{}",
                    songs.iter().fold(String::new(), |s, song| {
                        format!(
                            "{s}\n{} - {}",
                            time::format(song.metadata().duration(), 3),
                            song.metadata().title()
                        )
                    })
                )
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    // shared::song::convert_local("D:/dev/rust/projects/rmp/songs/", "D:/dev/rust/projects/rmp/songs/")

    socket.send(shared::message::ClientMessage::Exit).unwrap();
}
