use std::collections::BinaryHeap;

use difference::{Changeset, Difference};
use distance::damerau_levenshtein;
use serde::Serialize;

use crate::kampu::*;

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  selsisku: String,
  porsi: Vec<Valsi>,
}

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    use colored::*;
    let mut lerpoi = String::new();

    let cisni = self.porsi.iter().map(|x| x.cmene.len()).max().unwrap_or(0);

    lerpoi +=
      &format!("{} results found.\n", self.porsi.len().to_string().blue());
    lerpoi += "----\n";

    for valsi in &self.porsi {
      let mut frica = Changeset::new(&self.selsisku, &valsi.cmene, "");
      frica.diffs.retain(|x| match x {
        Difference::Rem(_) => false,
        _ => true,
      });

      lerpoi += &format!(
        "{:cisni$} - {}\n",
        frica,
        valsi.glosa.as_ref().unwrap_or(&"".into()),
        cisni = cisni + 1
      );
    }

    lerpoi
  }
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) {
  let selsisku = value_t!(selruhe, "word", String).unwrap();
  let velvihu_klani = value_t!(selruhe, "count", usize).unwrap();

  let mut indice = BinaryHeap::new();

  for (xo, vla) in vanbi.vlacku().sorcu.iter().enumerate() {
    let kaicla = damerau_levenshtein(&vla.cmene, &selsisku) as i32;
    indice.push((-kaicla, xo))
  }

  let mut porsi = Vec::new();
  for _ in 1..velvihu_klani {
    match indice.pop() {
      Some((_, xo)) => porsi.push(vanbi.vlacku().sorcu[xo].clone()),
      _ => (),
    }
  }

  let teryruhe = Teryruhe { selsisku, porsi };
  teryruhe.prina(vanbi).ok();
}
