#[macro_use]
extern crate clap;

use clap::App;
use kvs::KvStore;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut store = KvStore::new();
    match matches.subcommand_name() {
        Some("set") => {
            let m = matches.subcommand_matches("set").unwrap();
            let key = m.value_of("KEY").unwrap();
            let value = m.value_of("VALUE").unwrap();
            println!("set {} => {}", key, value);
            store.set(key.to_string(), value.to_string());
        },
        Some("get") => {
            let m = matches.subcommand_matches("get").unwrap();
            let key = m.value_of("KEY").unwrap();
            println!("get {}", key);
            let result = store.get(key.to_string()).unwrap_or("".to_string());
            println!("result => {}", result);
        },
        Some("rm") => {
            let m = matches.subcommand_matches("rm").unwrap();
            let key = m.value_of("KEY").unwrap();
            println!("remove {}", key);
            store.remove(key.to_string());
        },
        None => {},
        _ => {},
    }
}