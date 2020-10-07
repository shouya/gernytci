use std::convert::TryFrom;

use anyhow::{Error, Result};
use clap::{load_yaml, App};

mod kampu;
mod reltai;
mod sidju;
mod tutci;
mod vanbi;
mod vlacku;
mod gerna;

pub use reltai::Reltai;
pub use vanbi::Vanbi;
pub use vlacku::{Valsi, Vlacku};

fn main() -> Result<(), Error> {
  use tutci::*;

  let lahe_la_yaml = load_yaml!("../cli.yaml");
  let tercuha = App::from(lahe_la_yaml);
  let kampu_selcuha = tercuha.clone().get_matches();

  let vanbi = Vanbi::try_from(&kampu_selcuha)?;
  let minde = kampu_selcuha.subcommand_name().unwrap();
  let selcuha = kampu_selcuha.subcommand_matches(minde).unwrap();

  match minde {
    "hello" => coi::pruce(selcuha, &vanbi)?.prina(&vanbi)?,
    "search" => zvafahi::pruce(selcuha, &vanbi)?.prina(&vanbi)?,
    "tamsmi" => tamsmi::pruce(selcuha, &vanbi)?.prina(&vanbi)?,
    "cut" => katna::pruce(selcuha, &vanbi)?.prina(&vanbi)?,
    "convert" => bixygau::pruce(selcuha, &vanbi)?.prina(&vanbi)?,
    "lujvo" => jvofihi::pruce(selcuha, &vanbi)?.prina(&vanbi)?,
    _ => panic!("unreachable"),
  };

  Ok(())
}
