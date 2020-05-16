use std::path::Path;

use crate::kampu::*;
use crate::sidju;

use serde::{Deserialize, Serialize};
use serde_json;

type Selmaho = String;
type Rafsi = String;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Valsi {
  pub cmene: String,
  pub glosa: Option<String>,
  pub velciksi: Option<String>,
  pub selmaho: Selmaho,
  pub rafsi: Vec<Rafsi>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Vlacku {
  pub sorcu: Vec<Valsi>,
}

impl Vlacku {
  pub fn tolsorcu(pluta: &Path) -> Result<Self> {
    let sfaile_xadni = sidju::tolsorcu_sfaile(pluta)?;
    Ok(serde_json::from_str(&sfaile_xadni)?)
  }

  pub fn sorcu(&self, pluta: &Path) -> Result<()> {
    let lerpoi = serde_json::to_string(self)?;
    sidju::sorcu_sfaile(&pluta, &lerpoi)?;
    Ok(())
  }

  pub fn jmina(&mut self, valsi: Valsi)  {
    self.sorcu.push(valsi)
  }

  // pu'o zukte
  pub fn nerbei(pluta: &Path) -> Result<Self> {
    unimplemented!()
  }
}
