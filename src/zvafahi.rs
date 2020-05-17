use std::collections::HashMap;

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
    default_value = "5"
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

static RO_CKAJI: &[&str] = &["cmene", "glosa", "smuni", "pinka"];

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  selsisku: String,
  morna: Morna,
  zvati: HashMap<&'static str, Vec<Valsi>>,
}

pub fn zvafahi(tergaf: &crate::Tergalfi, vlacku: &Vlacku) -> Result<Teryruhe> {
  use crate::Minde::Zvafahi;

  let zvafahi_tergaf = match &tergaf.minde {
    Zvafahi(da) => da,
    _ => unreachable!(),
  };
  let morna = zbasu_morna(&zvafahi_tergaf.selsisku);
  let mut zvati: HashMap<&str, _> = HashMap::new();

  for ckaji in RO_CKAJI.iter() {
    zvati.insert(ckaji, Vec::new());
  }

  for valsi in vlacku.sorcu.iter() {
    for ckaji in RO_CKAJI {
      let vlamei = valsi.cpacu(ckaji).unwrap_or("".into());
      let mat = mapti(&morna, &vlamei);
      if mat.selkanpe.len() > 0 {
        zvati.get_mut(ckaji).unwrap().push((valsi.clone(), mat));
        break;
      }
    }
  }

  for liste in zvati.values_mut() {
    liste.sort_by(|da, de| de.1.vamji.partial_cmp(&da.1.vamji).unwrap());
    liste.truncate(zvafahi_tergaf.klani);
  }

  let zvati = zvati
    .into_iter()
    .map(|(ckiku, dacti)| {
      (ckiku, dacti.into_iter().map(|(da, _)| da).collect())
    })
    .collect();

  Ok(Teryruhe {
    selsisku: zvafahi_tergaf.selsisku.clone(),
    morna: morna,
    zvati: zvati,
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
    prina_pagbu("Word", &self.zvati["cmene"], &self.morna);
    prina_pagbu("Gloss", &self.zvati["glosa"], &self.morna);
    prina_pagbu("Definition", &self.zvati["smuni"], &self.morna);
    prina_pagbu("Notes", &self.zvati["pinka"], &self.morna);
  }
  fn termontai_lahe_jeison(&self) {
    println!("{}", serde_json::to_string(self).unwrap());
  }
}

fn prina_pagbu(pagbu_cmene: &str, liste: &Vec<Valsi>, morna: &Morna) {
  if liste.len() == 0 {
    return;
  }

  println!(
    "{} results matching {}",
    liste.len().to_string().blue(),
    pagbu_cmene.bold().blue()
  );

  for vla in liste {
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
