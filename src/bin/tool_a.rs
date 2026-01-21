use clap::Arg;

pub fn build_app() -> clap::Command {
    clap::Command::new("myapp")
        .version("1.0")
        .about("An example CLI application")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Increases verbosity"),
        )
}

fn main() {
    let _matches = build_app().get_matches();
}
