use distance::levenshtein;
use serde::Serialize;
use serde_json;
use structopt::StructOpt;

use crate::kampu::*;
use crate::vlacku::{Valsi, Vlacku};

#[derive(Debug, StructOpt)]
#[structopt(name = "tamsmi", about = "Find words similar in shape")]
pub struct Tergalfi {
  #[structopt(
    name = "count",
    short,
    long,
    about = "Number of results to return",
    default_value = "10"
  )]
  klani: usize,

  #[structopt(name = "word")]
  selsisku: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  selsisku: String,
  porsi: Vec<Valsi>,
}

impl crate::TciTeryruhe for Teryruhe {
  fn termontai_lo_vlamei(&self) {
    use colored::*;

    let cisni = self.porsi.iter().map(|x| x.cmene.len()).max().unwrap_or(0);

    println!("{} results found.", self.porsi.len().to_string().blue());

    for valsi in &self.porsi {
      println!(
        "{:cisni$} - {}",
        valsi.cmene.green(),
        valsi.glosa.as_ref().unwrap_or(&"".into()),
        cisni = cisni + 2
      );
    }
  }

  fn termontai_lahe_jeison(&self) {
    println!("{}", serde_json::to_string(self).unwrap())
  }
}

pub fn tamsmi(tergaf: &crate::Tergalfi, vlacku: &Vlacku) -> Result<Teryruhe> {
  use crate::Minde::Tamsmi;

  let tamsmi_tergaf = match &tergaf.minde {
    Tamsmi(da) => da,
    _ => unreachable!(),
  };
  let selsisku = tamsmi_tergaf.selsisku.clone();
  let mut porsi = vlacku.sorcu.clone();

  porsi.sort_by_key(|v| levenshtein(&v.cmene, &selsisku));
  porsi.truncate(tamsmi_tergaf.klani);

  Ok(Teryruhe { selsisku, porsi })
}
