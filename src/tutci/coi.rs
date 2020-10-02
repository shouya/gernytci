use clap::{App, ArgMatches};
use crate::Vanbi;

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) {
  if selruhe.is_present("coho") {
    println!("co'o")
  } else {
    println!("coi")
  }
}
