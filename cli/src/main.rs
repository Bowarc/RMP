mod cmd_parser;
mod commands;

#[macro_use]
extern crate log; // trace, debug, info, warn, error

fn main() {
    // let cfg = logger::LoggerConfig::default()
    //     .add_filter("symphonia_core", log::LevelFilter::Off)
    //     .add_filter("symphonia_bundle_mp3", log::LevelFilter::Off)
    //     .add_filter("networking", log::LevelFilter::Debug);
    // logger::init(cfg, Some("./log/server.log"));

    let mut client = shared::client::Client::new().unwrap();

    match cmd_parser::cmd().get_matches().subcommand() {
        Some(("player", args)) => match args.subcommand() {
            Some(("play", _args)) => commands::player::play(&mut client),
            Some(("pause", _args)) => commands::player::pause(&mut client),
            Some(("now_playing", _args)) => {
                let (song, index) = commands::player::now_playing(&mut client);

                debug!("Currently playing ({index}) {title}", title = song.metadata().title());
            }
            Some(("queue", args)) => match args.subcommand() {
                Some(("get", _args)) => {
                    let queue = commands::player::queue::get(&mut client);

                    debug!("{:?}", queue.iter().map(|s| s.metadata().title().clone()).collect::<Vec<_>>());
                }
                Some(("add", args)) => {
                    use std::str::FromStr as _;
                    let arg = args.get_one::<String>("ARG").unwrap();
                    let si = if let Ok(uuid) = uuid::Uuid::from_str(arg) {
                        commands::player::queue::SongIdentifier::Uuid(uuid)
                    } else {
                        commands::player::queue::SongIdentifier::Title(arg.to_owned())
                    };
                    commands::player::queue::add(&mut client, si);
                }
                Some(("remove", args)) => {
                    use std::str::FromStr as _;
                    let arg = args.get_one::<String>("ARG").unwrap();
                    let si = if let Ok(uuid) = uuid::Uuid::from_str(arg) {
                        commands::player::queue::SongIdentifier::Uuid(uuid)
                    } else {
                        commands::player::queue::SongIdentifier::Title(arg.to_owned())
                    };
                    commands::player::queue::remove(&mut client, si);
                }
                Some(("clear", _args)) => commands::player::queue::clear(&mut client),
                _ => unreachable!(),
            },
            Some(("volume", args)) => match args.subcommand() {
                Some(("get", _args)) => {
                    let out = commands::player::volume::get(&mut client);
                    debug!("{out}")
                }
                Some(("set", args)) => {
                    let amount = args.get_one::<f32>("VALUE").unwrap();
                    commands::player::volume::set(&mut client, *amount);
                }
                Some(("up", args)) => {
                    let amount = args.get_one::<f32>("AMNT").unwrap();
                    commands::player::volume::up(&mut client, *amount)
                }
                Some(("down", args)) => {
                    let amount = args.get_one::<f32>("AMNT").unwrap();
                    commands::player::volume::down(&mut client, *amount)
                }
                _ => unreachable!(),
            },
            Some(("position", args)) => match args.subcommand(){
                Some(("get", _args)) => {
                    let pos = commands::player::position::get(&mut client);
                    debug!("Player's currently at {pos:?}");
                },
                Some(("set", args)) => {
                    let position_s = args.get_one::<u64>("VALUE").unwrap();

                    commands::player::position::set(&mut client, std::time::Duration::from_secs(*position_s))
                },
                Some(("forward", args)) => {
                    let position_s = args.get_one::<u64>("AMNT").unwrap();

                    commands::player::position::forward(&mut client, std::time::Duration::from_secs(*position_s))
                },
                Some(("backward", args)) => {
                    let position_s = args.get_one::<u64>("AMNT").unwrap();

                    commands::player::position::backward(&mut client, std::time::Duration::from_secs(*position_s))
                },
                _ => unreachable!()
            }
            Some(("device", args)) => match args.subcommand() {
                Some(("get", _args)) => {
                    let name = commands::player::device::get(&mut client);
                    debug!("{name}")
                }
                Some(("set", args)) => {
                    let device_name = args.get_one::<String>("NAME").unwrap();
                    commands::player::device::set(&mut client, device_name.to_owned())
                }
                _ => unreachable!(),
            },

            _ => unreachable!(),
        },
        Some(("downloader", args)) => {
            match args.subcommand() {
                Some(("start", args)) => {
                    let url = args.get_one::<String>("URL").unwrap(); // TODO: Take care of this one
                    commands::downloader::download(&mut client, url.to_owned());
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }


    // shared::song::convert_local("D:/dev/rust/projects/rmp/songs/", "D:/dev/rust/projects/rmp/songs/")

    client.shutdown();


}
