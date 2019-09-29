#[macro_use]
extern crate clap;

use clap::App;
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("set") => {
            eprintln!("unimplemented");
            process::exit(-1);
        }
        Some("get") => {
            eprintln!("unimplemented");
            process::exit(-1);
        }
        Some("rm") => {
            eprintln!("unimplemented");
            process::exit(-1);
        }
        None => {
            process::exit(-1);
        }
        _ => {
            process::exit(-1);
        }
    }
}
