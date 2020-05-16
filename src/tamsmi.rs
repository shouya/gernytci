use distance::levenshtein;
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
  terkancu: usize,

  #[structopt(name = "word")]
  selsisku: String,
}

pub struct Teryruhe {
  selsisku: String,
  porsi: Vec<Valsi>,
}

pub fn tamsmi(
  tergaf: &crate::Tergalfi,
  vlacku: &Vlacku,
) -> Result<Teryruhe> {
  use crate::Minde::Tamsmi;

  let Tamsmi(tamsmi_tergaf) = &tergaf.minde;
  let selsisku = tamsmi_tergaf.selsisku.clone();
  let mut porsi = vlacku.sorcu.clone();
  porsi.sort_by_key(|v| levenshtein(&v.cmene, &selsisku));
  porsi.truncate(tamsmi_tergaf.terkancu);
  Ok(Teryruhe { selsisku, porsi })
}
