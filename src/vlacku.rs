use std::cell::RefCell;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use crate::sidju;

use anyhow::{anyhow, Result};
use clap::ArgMatches;
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

impl Valsi {
  pub fn cpacu(&self, ckaji: &str) -> Option<String> {
    match ckaji {
      "cmene" => Some(self.cmene.clone()),
      "klesi" => Some(self.klesi.clone()),
      "selmaho" => self.selmaho.clone(),
      "glosa" => self.glosa.clone(),
      "smuni" => self.smuni.clone(),
      "rafsi" => None,
      "krasi" => Some(self.krasi.clone()),
      "pinka" => self.pinka.clone(),
      _ => None,
    }
  }
}

#[derive(Debug)]
pub enum LazniVlacku {
  Uonai { catni_poho: bool, sfaile: PathBuf },
  Uo(Vlacku),
}

impl TryFrom<&ArgMatches<'_>> for LazniVlacku {
  type Error = anyhow::Error;

  fn try_from(selcuha: &ArgMatches<'_>) -> Result<LazniVlacku> {
    let sfaile: PathBuf = selcuha
      .value_of("dict")
      .ok_or(anyhow!("Dictionary not specified"))?
      .into();
    let catni_poho = selcuha.is_present("official-only");

    Ok(LazniVlacku::Uonai { catni_poho, sfaile })
  }
}

impl LazniVlacku {
  fn tolsorcu(&mut self) -> Result<()> {
    use LazniVlacku::{Uo, Uonai};
    match self {
      Uo(_) => (),
      Uonai { catni_poho, sfaile } => {
        let mut vlacku = Vlacku::tolsorcu(&sfaile)?;
        if *catni_poho {
          vlacku.catni_poho()
        }

        *self = Uo(vlacku);
        ()
      }
    }

    Ok(())
  }

  pub fn cpacu(&mut self) -> Result<&mut Vlacku> {
    self.tolsorcu()?;
    match self {
      Self::Uo(vlacku) => Ok(vlacku),
      _ => Err(anyhow!("Failed to load dictionary!")),
    }
  }
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
      indice: RefCell::new(None),
    })
  }

  pub fn catni_poho(&mut self) {
    self.sorcu.retain(|x| x.krasi == "officialdata")
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
