use crate::Vanbi;
use clap::ArgMatches;

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) {
  if selruhe.is_present("coho") {
    println!("co'o")
  } else {
    println!("coi");
    println!("{:?}", vanbi.vlacku().zvafahi("coi"))
  }
}
