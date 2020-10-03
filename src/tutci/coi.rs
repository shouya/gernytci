use crate::kampu::*;

#[derive(Debug, Serialize)]
struct Coi;

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<impl Reltai> {
  let le_selrinsa = selruhe.value_of("target").unwrap_or("munje");
  if selruhe.is_present("goodbye") {
    println!("co'o {}", le_selrinsa)
  } else {
    println!("{:?}", vanbi.vlacku()?.zvafahi("coi"));
    println!("coi {}", le_selrinsa);
  }

  Ok(Coi)
}

impl ToString for Coi {
  fn to_string(&self) -> String {
    format!("{:?}", self)
  }
}
