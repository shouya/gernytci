use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::kampu::*;
use crate::sidju;

use serde::{Deserialize, Serialize};
use simd_json;

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

#[derive(Clone, Debug)]
pub struct Vlacku {
  pub sorcu: Vec<Valsi>,
  pub pluta: PathBuf,
  pub indice: RefCell<Option<BTreeMap<String, usize>>>,
}

impl Vlacku {
  pub fn tolsorcu(pluta: &Path) -> Result<Self> {
    let mut sfaile_xadni = if pluta.to_str().unwrap() == "[built-in]" {
      sidju::jinzi_vlacku_sfaile()?
    } else if Path::exists(pluta) {
      sidju::tolsorcu_sfaile(pluta)?
    } else {
      "[]".into()
    };

    let sorcu = simd_json::from_str(&mut sfaile_xadni)?;

    Ok(Self {
      sorcu: sorcu,
      pluta: pluta.into(),
      indice: RefCell::new(None)
    })
  }

  pub fn sorcu(&self) -> Result<()> {
    let lerpoi = serde_json::to_string(&self.sorcu)?;
    sidju::sorcu_sfaile(&self.pluta, &lerpoi)?;
    Ok(())
  }

  #[allow(dead_code)]
  pub fn zvafahi(&self, cmene: &str) -> Option<Valsi> {
    self.zbasu_indice();
    self
      .indice
      .borrow()
      .as_ref()
      .unwrap()
      .get(cmene)
      .and_then(|xo| self.sorcu.get(*xo))
      .map(|da| da.clone())
  }

  pub fn zbasu_indice(&self) {
    if self.indice.borrow().is_none() {
      let mut indice_zbasu = BTreeMap::new();
      for (xo, vla) in self.sorcu.iter().enumerate() {
        indice_zbasu.insert(vla.cmene.clone(), xo);
      }
      self.indice.replace(Some(indice_zbasu));
    }
  }
}
