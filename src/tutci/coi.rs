use clap::ArgMatches;
use crate::Vanbi;

pub fn pruce(selruhe: &ArgMatches, _vanbi: &Vanbi) {
  if selruhe.is_present("coho") {
    println!("co'o")
  } else {
    println!("coi")
  }
}
