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

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<impl Reltai> {
  let selsisku = value_t!(selruhe, "word", String).unwrap();
  let velvihu_klani = value_t!(selruhe, "count", usize).unwrap();

  let mut indice = BinaryHeap::new();

  for vla in vanbi.vlacku()?.iter() {
    let nilsmi = damerau_levenshtein(&vla.cmene, &selsisku) as i32;
    indice.push((nilsmi, vla.clone()));
    if indice.len() > velvihu_klani {
      indice.pop();
    }
  }

  let porsi = indice
    .into_sorted_vec()
    .into_iter()
    .map(|(_nilsmi, vla)| vla)
    .collect();

  Ok(Teryruhe { selsisku, porsi })
}
