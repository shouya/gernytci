use crate::Vanbi;
use clap::ArgMatches;

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) {
  let le_selrinsa = selruhe.value_of("target").unwrap_or("munje");
  if selruhe.is_present("goodbye") {
    println!("co'o {}", le_selrinsa)
  } else {
    println!("{:?}", vanbi.vlacku().zvafahi("coi"));
    println!("coi {}", le_selrinsa);
  }
}
