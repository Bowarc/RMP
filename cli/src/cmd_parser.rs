pub fn cmd() -> clap::Command {
    use clap::{arg, Command};

    Command::new("CLI client")
        .about("CLI client for RMP")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // .allow_external_subcommands(yes)
        .subcommand(
            Command::new("player")
            .about("Any command that use the music player")
    	    .subcommand(Command::new("play").alias("p"))
    	    .subcommand(Command::new("pause").alias("!p"))
            .subcommand(
            Command::new("queue").alias("q")
                .about("Modify the RMP song queue")
                .subcommand(
                	Command::new("add").alias("a")
            		.about("Adds a given song name or UUID to RMP's song queue")
            		.arg(arg!(<ARG> "The UUID or name of the requested song")
            			.help("Must be a valid song name or v4 UUID, see <https://en.wikipedia.org/wiki/Universally_unique_identifier> for more information"))
            		.arg_required_else_help(true),
                )
                .subcommand(
                	Command::new("remove").alias("d")
            		.about("Removes a given song name or UUID to RMP's song queue")
            		.arg(arg!(<ARG> "The UUID or name of the requested song")
            			.help("Must be a valid song name or v4 UUID, see <https://en.wikipedia.org/wiki/Universally_unique_identifier> for more information"))
            		.arg_required_else_help(true),
                )
                .subcommand(
                	Command::new("clear").alias("c")
            		.about("Clear RMP's song queue")
                )
	        )
            .subcommand(
            Command::new("volume")
                .alias("vol")
                .subcommand(
                    Command::new("get")
                    .alias("g")
                )
                .subcommand(
                    Command::new("set")
                    .alias("s")
                    .arg(
                        arg!(<VALUE> "The amount as float")
                        .value_parser(clap::value_parser!(f32))
                    )
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("up")
                    .alias("u")
                    .arg(
                        arg!(<AMNT> "The amount as float")
                        .required(false)
                        .value_parser(clap::value_parser!(f32))
                        .default_value("0.5")
                    )
                )
                .subcommand(
                    Command::new("down")
                    .alias("d")
                    .arg(
                        arg!(<AMNT> "The amount as float")
                        .required(false)
                        .value_parser(clap::value_parser!(f32))
                        .default_value("0.5")
                    )
                )
            )
            .subcommand(
            Command::new("position")
                .alias("pos")
                .about("Sets the positon of the sound currently playing")
                .subcommand(
                    Command::new("get")
                    .alias("g")
                )
                .subcommand(
                    Command::new("set")
                    .alias("s")
                    .arg(
                        arg!(<VALUE> "The amount of seconds")
                        .value_parser(clap::value_parser!(u64))
                    )
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("forward")
                    .alias("f")
                    .arg(
                        arg!(<AMNT> "The amount of seconds")
                        .required(false)
                        .value_parser(clap::value_parser!(u64))
                        .default_value("15")
                    )
                )
                .subcommand(
                    Command::new("backwards")
                    .alias("b")
                    .arg(
                        arg!(<AMNT> "The amount of seconds")
                        .required(false)
                        .value_parser(clap::value_parser!(u64))
                        .default_value("15")
                    )
                )
            )
            .subcommand(
            Command::new("device")
                .alias("d")
                .subcommand(
                    Command::new("get")
                    .alias("g")
                    .about("Retrieve the name of the currently set device")
                )
                .subcommand(
                    Command::new("set")
                    .alias("s")
                    .about("Sets a new device by name")
                    .arg(arg!(<NAME> "Name of the device to set"))
                    .arg_required_else_help(true)
                )
            )
        )
        .subcommand(
            Command::new("downloader").alias("dl")
            .subcommand(
                Command::new("start")
                    .arg(arg!(<URL> "The url used for the download, must be youtube.com/"))
                    // .value_parser(|v| )
                )
        )
}
