use anyhow::Error;
use clap::{load_yaml, App};

mod reltai;
mod sidju;
mod tutci;
mod vanbi;
mod vlacku;

use reltai::Reltai;
use vanbi::Vanbi;
use vlacku::Valsi;

fn main() -> Result<(), Error> {
  use tutci::*;

  let lahe_la_yaml = load_yaml!("../cli.yaml");
  let mut tercuha = App::from(lahe_la_yaml);
  let kampu_selcuha = tercuha.clone().get_matches();

  let vanbi = Vanbi::new(&kampu_selcuha);

  match kampu_selcuha.subcommand() {
    ("hello", Some(selcuha)) => coi::pruce(selcuha, &vanbi),
    ("search", Some(selcuha)) => zvafahi::pruce(selcuha, &vanbi),
    _ => {
      println!("Invalid command!\n");
      tercuha.print_help()?;
      println!("")
    }
  }

  Ok(())
}
