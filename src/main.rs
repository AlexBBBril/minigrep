use minigrep::Config;
use std::process;

fn main() {
    let config = Config::parse_config();

    let config = Config::build(config).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
