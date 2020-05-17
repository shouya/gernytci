use std::collections::BinaryHeap;

use difference::{Difference, Changeset};
use distance::damerau_levenshtein;
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
    help = "Number of results to return",
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
    println!("----");

    for valsi in &self.porsi {
      let mut frica = Changeset::new(&self.selsisku, &valsi.cmene, "");
      frica.diffs.retain(|x| match x {
        Difference::Rem(_) => false,
        _ => true,
      });

      println!(
        "{:cisni$} - {}",
        frica,
        valsi.glosa.as_ref().unwrap_or(&"".into()),
        cisni = cisni + 1
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

  let mut indice = BinaryHeap::new();

  for (xo, vla) in vlacku.sorcu.iter().enumerate() {
    let kaicla = damerau_levenshtein(&vla.cmene, &selsisku) as i32;
    indice.push((-kaicla, xo))
  }

  let mut porsi = Vec::new();
  for _ in 1..tamsmi_tergaf.klani {
    match indice.pop() {
      Some((_, xo)) => porsi.push(vlacku.sorcu[xo].clone()),
      _ => (),
    }
  }

  Ok(Teryruhe { selsisku, porsi })
}
