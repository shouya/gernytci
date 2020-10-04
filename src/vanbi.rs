use std::cell::RefCell;
use std::convert::TryFrom;
use std::ops::Deref;
use std::path::PathBuf;

use anyhow::Result;
use clap::ArgMatches;

use crate::vlacku::{LazniVlacku, Vlacku};

pub enum PrinaTarmi {
  Json,
  Text,
}

impl From<&ArgMatches<'_>> for PrinaTarmi {
  fn from(selcuha: &ArgMatches<'_>) -> Self {
    match selcuha.value_of("format") {
      Some("json") => PrinaTarmi::Json,
      Some("text") => PrinaTarmi::Text,
      _ => panic!("Invalid output format"),
    }
  }
}

pub struct Vanbi {
  pub prina_tarmi: PrinaTarmi,
  vlacku: RefCell<LazniVlacku>,
}

impl TryFrom<&ArgMatches<'_>> for Vanbi {
  type Error = anyhow::Error;

  fn try_from(selcuha: &ArgMatches<'_>) -> Result<Self> {
    Ok(Vanbi {
      prina_tarmi: PrinaTarmi::from(selcuha),
      vlacku: RefCell::new(LazniVlacku::try_from(selcuha)?),
    })
  }
}

impl Vanbi {
  pub fn vlacku(&self) -> Result<impl Deref<Target = Vlacku>> {
    Ok(self.vlacku.borrow_mut().cpacu()?)
  }

  pub fn vlacku_sfaile(&self) -> PathBuf {
    self.vlacku.borrow().sfaile().into()
  }
}
