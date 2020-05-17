use colored::*;
use itertools::Itertools;
use serde::Serialize;
use structopt::StructOpt;

use crate::kampu::*;
use crate::vlacku::{Valsi, Vlacku};

#[derive(Debug, StructOpt)]
#[structopt(name = "zvafahi", about = "Lookup dictionary")]
pub struct Tergalfi {
  #[structopt(
    name = "count",
    short,
    long,
    help = "Number of results to return",
    default_value = "10"
  )]
  klani: usize,

  #[structopt(name = "text")]
  selsisku: String,
}

#[derive(Clone, Serialize, Debug)]
struct Mapti {
  selkanpe: Vec<String>,
  vamji: f32,
}

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  selsisku: String,
  morna: Morna,
  zvati_cmene: Vec<(Valsi, Mapti)>,
  zvati_glosa: Vec<(Valsi, Mapti)>,
  zvati_smuni: Vec<(Valsi, Mapti)>,
  zvati_pinka: Vec<(Valsi, Mapti)>,
}

pub fn zvafahi(tergaf: &crate::Tergalfi, vlacku: &Vlacku) -> Result<Teryruhe> {
  use crate::Minde::Zvafahi;

  let zvafahi_tergaf = match &tergaf.minde {
    Zvafahi(da) => da,
    _ => unreachable!(),
  };
  let morna = zbasu_morna(&zvafahi_tergaf.selsisku);
  let mut zvati_cmene = Vec::new();
  let mut zvati_glosa = Vec::new();
  let mut zvati_smuni = Vec::new();
  let mut zvati_pinka = Vec::new();

  let troci_mapti =
    |vasru: &mut Vec<_>, selzvati: &Option<String>, valsi: &Valsi| {
      let mat = mapti(&morna, &selzvati.as_ref().unwrap_or(&"".into()));
      if mat.selkanpe.len() > 0 {
        vasru.push((valsi.clone(), mat));
        true
      } else {
        false
      }
    };

  for valsi in vlacku.sorcu.iter() {
    let Valsi {
      cmene,
      glosa,
      smuni,
      pinka,
      ..
    } = valsi;

    if troci_mapti(&mut zvati_cmene, &Some(cmene.clone()), valsi) {
      continue;
    }
    if troci_mapti(&mut zvati_glosa, glosa, valsi) {
      continue;
    }
    if troci_mapti(&mut zvati_smuni, smuni, valsi) {
      continue;
    }
    if troci_mapti(&mut zvati_pinka, pinka, valsi) {
      continue;
    }
  }

  zvati_cmene.sort_by(|da, de| de.1.vamji.partial_cmp(&da.1.vamji).unwrap());
  zvati_glosa.sort_by(|da, de| de.1.vamji.partial_cmp(&da.1.vamji).unwrap());
  zvati_smuni.sort_by(|da, de| de.1.vamji.partial_cmp(&da.1.vamji).unwrap());
  zvati_pinka.sort_by(|da, de| de.1.vamji.partial_cmp(&da.1.vamji).unwrap());

  zvati_cmene.truncate(zvafahi_tergaf.klani);
  zvati_glosa.truncate(zvafahi_tergaf.klani);
  zvati_smuni.truncate(zvafahi_tergaf.klani);
  zvati_pinka.truncate(zvafahi_tergaf.klani);

  Ok(Teryruhe {
    selsisku: zvafahi_tergaf.selsisku.clone(),
    morna,
    zvati_cmene,
    zvati_glosa,
    zvati_smuni,
    zvati_pinka,
  })
}

type Morna = Vec<(String, f32)>;

pub fn zbasu_morna(selsisku: &str) -> Morna {
  let mut jalge = Vec::new();
  for (da, de) in selsisku.split_whitespace().into_iter().tuple_windows() {
    jalge.push((format!("{} {}", da, de), 2.0));
  }

  for da in selsisku.split_whitespace() {
    jalge.push((da.into(), 1.0));
  }

  for (da, junta) in jalge.clone() {
    if da.contains('\'') {
      jalge.push((da.replace('\'', "h"), junta));
    }
    if da.contains('h') {
      jalge.push((da.replace('h', "'"), junta));
    }
  }

  jalge.dedup_by(|da, de| da.0.contains(&de.0));
  jalge.sort_by_key(|da| da.0.len() as i32 * -1);

  jalge
}

fn mapti(morna: &Morna, selzvati: &str) -> Mapti {
  let mut vamji = 0.0;
  let mut selkanpe = Vec::new();

  if selzvati.is_empty() {
    return Mapti { vamji, selkanpe };
  }

  for (zvati, junta) in morna {
    let zvati = zvati.to_ascii_lowercase();
    let selkancu = selzvati.to_ascii_lowercase().matches(&zvati).count();
    if selkancu > 0 {
      vamji += junta * selkancu as f32 / (1.0 + selzvati.len() as f32).ln();
      selkanpe.push(zvati.into())
    }
  }

  Mapti { vamji, selkanpe }
}

impl crate::TciTeryruhe for Teryruhe {
  fn termontai_lo_vlamei(&self) {
    prina_pagbu("Word", &self.zvati_cmene, &self.morna);
    prina_pagbu("Gloss", &self.zvati_glosa, &self.morna);
    prina_pagbu("Definition", &self.zvati_smuni, &self.morna);
    prina_pagbu("Notes", &self.zvati_pinka, &self.morna);
  }
  fn termontai_lahe_jeison(&self) {
    println!("{}", serde_json::to_string(self).unwrap());
  }
}

fn prina_pagbu(pagbu_cmene: &str, liste: &Vec<(Valsi, Mapti)>, morna: &Morna) {
  if liste.len() == 0 {
    return;
  }

  println!(
    "{} results matching {}",
    liste.len().to_string().blue(),
    pagbu_cmene.bold().blue()
  );

  for (vla, _mat) in liste {
    prina_valsi(vla, morna);
  }

  println!("\n----\n")
}

fn prina_valsi(valsi: &Valsi, morna: &Morna) {
  let rafsi_lerpoi = format!("-{}-", valsi.rafsi.join("-"));
  let rafsi_pagbu = if rafsi_lerpoi != "--" {
    format!("{} ", skagau_lerpoi(&rafsi_lerpoi, morna))
  } else {
    "".into()
  };

  print!(
    "\n{} {:35} ",
    "*".purple(),
    format!(
      "{} {}",
      valsi.cmene.underline().bold().purple(),
      rafsi_pagbu
    )
  );

  if let Some(glosa) = &valsi.glosa {
    print!("{:20} ", format!("{}", skagau_lerpoi(&glosa, morna).cyan()));
  }

  if let Some(selmaho) = &valsi.selmaho {
    print!("{:>7} ", selmaho);
  }
  print!("{:20} ", format!("({})", valsi.klesi.green()));
  print!("{:10} ", format!("[@{}]", valsi.krasi));
  println!("");

  if let Some(smuni) = &valsi.smuni {
    print!("  {}", skagau_lerpoi(smuni, morna));
  }
  if let Some(pinka) = &valsi.pinka {
    println!("\n");
    print!("  {}", skagau_lerpoi(pinka, morna));
  }
  println!("")
}

fn skagau_lerpoi(lerpoi: &str, morna: &Morna) -> String {
  let mut jalge = lerpoi.to_string();
  for (mapti, _) in morna {
    jalge = jalge.replace(mapti, &mapti.on_blue().to_string());
  }
  jalge
}

//   selsisku: String,
//   zvati_cmene: Vec<(Valsi, Mapti)>,
//   zvati_glosa: Vec<(Valsi, Mapti)>,
//   zvati_smuni: Vec<(Valsi, Mapti)>,
//   zvati_pinka: Vec<(Valsi, Mapti)>,
// }
