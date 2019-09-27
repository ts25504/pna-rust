#[macro_use]
extern crate clap;
extern crate dotenv;

use std::path::Path;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    println!("Value for INPUT: {}", input);

    match matches.value_of("config") {
        Some(config) => print_env(config),
        None => {},
    };
}

fn print_env(config: &str) {
    println!("Value for config: {}", config);
    let config_path = Path::new(config);
    println!("Value for config: {}", config);
    dotenv::from_path(config_path).unwrap();
    let config_params: Vec<(String, String)> = dotenv::vars().collect();
    println!("=========================== Environment Variables ===========================");
    for (key, value) in config_params.iter() {
        println!("{} = {}", key, value);
    }
}
