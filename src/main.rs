use anyhow::Error;
use clap::{load_yaml, App};

mod tutci;
mod vanbi;
mod vlacku;
mod sidju;

use vanbi::Vanbi;

fn main() -> Result<(), Error> {
  use tutci::*;

  let lahe_la_yaml = load_yaml!("../cli.yaml");
  let mut tercuha = App::from(lahe_la_yaml);
  let selcuha = tercuha.clone().get_matches();

  let vanbi = Vanbi::new(&selcuha);

  match selcuha.subcommand() {
    ("coi", Some(mapti)) => coi::pruce(mapti, &vanbi),
    _ => {
      tercuha.print_help()?;
    }
  }

  Ok(())
}
