use std::path::{Path, PathBuf};

use crate::kampu::*;
use crate::sidju;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Valsi {
  pub cmene: String,
  pub klesi: String,
  pub selmaho: Option<String>,
  pub glosa: Option<String>,
  pub smuni: Option<String>,
  pub rafsi: Vec<String>,
  pub krasi: String,
  pub pinka: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Vlacku {
  pub sorcu: Vec<Valsi>,
  pub pluta: PathBuf,
}

impl Vlacku {
  pub fn tolsorcu(pluta: &Path) -> Result<Self> {
    if Path::exists(pluta) {
      let sfaile_xadni = sidju::tolsorcu_sfaile(pluta)?;
      Ok(Self {
        sorcu: serde_json::from_str(&sfaile_xadni)?,
        pluta: pluta.into(),
      })
    } else {
      Ok(Self {
        sorcu: Vec::new(),
        pluta: pluta.into(),
      })
    }
  }

  pub fn sorcu(&self) -> Result<()> {
    let lerpoi = serde_json::to_string(&self.sorcu)?;
    sidju::sorcu_sfaile(&self.pluta, &lerpoi)?;
    Ok(())
  }

  pub fn jmina(&mut self, valsi: Valsi) {
    self.sorcu.push(valsi)
  }
}
