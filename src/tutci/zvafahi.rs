use std::collections::HashMap;

use clap::{value_t, values_t, ArgMatches};
use colored::*;
use serde::Serialize;

use crate::{Reltai, Valsi, Vanbi};

#[derive(Clone, Serialize, Debug)]
struct Mapti {
  selkanpe: Vec<String>,
  vamji: f32,
}

static RO_CKAJI: &[&str] = &["cmene", "glosa", "smuni", "pinka"];

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  selsisku: Vec<String>,
  morna: Morna,
  zvati: HashMap<&'static str, Vec<Valsi>>,
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) {
  let selsisku = values_t!(selruhe, "keyword", String).unwrap();
  let velvihu_klani = value_t!(selruhe, "count", usize).unwrap();
  let morna = zbasu_morna(selsisku.as_slice());
  let mut zvati: HashMap<&str, _> = HashMap::new();

  for ckaji in RO_CKAJI.iter() {
    zvati.insert(ckaji, Vec::new());
  }

  for valsi in vanbi.vlacku().sorcu.iter() {
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
    liste.truncate(velvihu_klani);
  }

  let zvati = zvati
    .into_iter()
    .map(|(ckiku, dacti)| {
      (ckiku, dacti.into_iter().map(|(da, _)| da).collect())
    })
    .collect();

  let teryruhe = Teryruhe {
    selsisku: selsisku,
    morna: morna,
    zvati: zvati,
  };

  teryruhe.prina(vanbi).ok();
}

type Morna = Vec<(String, f32)>;

pub fn zbasu_morna(selsisku: &[String]) -> Morna {
  let mut jalge = Vec::new();

  for da in selsisku {
    jalge.push((da.to_string(), 1.0));
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

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    let mut lerpoi = String::new();
    lerpoi += &prina_pagbu("Word", &self.zvati["cmene"], &self.morna);
    lerpoi += &prina_pagbu("Gloss", &self.zvati["glosa"], &self.morna);
    lerpoi += &prina_pagbu("Definition", &self.zvati["smuni"], &self.morna);
    lerpoi += &prina_pagbu("Notes", &self.zvati["pinka"], &self.morna);
    lerpoi
  }
}

fn prina_pagbu(pagbu_cmene: &str, liste: &Vec<Valsi>, morna: &Morna) -> String {
  let mut lerpoi = String::new();
  if liste.len() == 0 {
    return lerpoi;
  }

  lerpoi += &format!(
    "{} results matching {}",
    liste.len().to_string().blue(),
    pagbu_cmene.bold().blue()
  );

  for vla in liste {
    lerpoi += &prina_valsi(vla, morna);
  }

  lerpoi += "\n----\n";

  lerpoi
}

fn prina_valsi(valsi: &Valsi, morna: &Morna) -> String {
  let mut lerpoi = String::new();
  let rafsi_lerpoi = format!("-{}-", valsi.rafsi.join("-"));
  let rafsi_pagbu = if rafsi_lerpoi != "--" {
    format!("{} ", skagau_lerpoi(&rafsi_lerpoi, morna))
  } else {
    "".into()
  };

  lerpoi += &format!(
    "\n{} {:35} ",
    "*".purple(),
    format!(
      "{} {}",
      valsi.cmene.underline().bold().purple(),
      rafsi_pagbu
    )
  );

  if let Some(glosa) = &valsi.glosa {
    lerpoi += &format!("{:20} ", skagau_lerpoi(&glosa, morna).cyan());
  }

  if let Some(selmaho) = &valsi.selmaho {
    lerpoi += &format!("{:>7} ", selmaho);
  }
  lerpoi += &format!("{:20} ", format!("({})", valsi.klesi.green()));
  lerpoi += &format!("{:10} ", format!("[@{}]", valsi.krasi));
  lerpoi += "\n";

  if let Some(smuni) = &valsi.smuni {
    lerpoi += &format!("  {}", skagau_lerpoi(smuni, morna));
  }
  if let Some(pinka) = &valsi.pinka {
    lerpoi += "\n\n";
    lerpoi += &format!("  {}", skagau_lerpoi(pinka, morna));
  }
  lerpoi += "\n";

  lerpoi
}

fn skagau_lerpoi(lerpoi: &str, morna: &Morna) -> String {
  let mut jalge = lerpoi.to_string();
  for (mapti, _) in morna {
    jalge = jalge.replace(mapti, &mapti.on_blue().to_string());
  }
  jalge
}
