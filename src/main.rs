use anyhow::Error;
use clap::{load_yaml, App};

mod sidju;
mod tutci;
mod vanbi;
mod vlacku;

use vanbi::Vanbi;

fn main() -> Result<(), Error> {
  use tutci::*;

  let lahe_la_yaml = load_yaml!("../cli.yaml");
  let mut tercuha = App::from(lahe_la_yaml);
  let selcuha = tercuha.clone().get_matches();

  let vanbi = Vanbi::new(&selcuha);

  match selcuha.subcommand() {
    ("hello", Some(mapti)) => coi::pruce(mapti, &vanbi),
    _ => {
      println!("Invalid command!\n");
      tercuha.print_help()?;
      println!("")
    }
  }

  Ok(())
}
