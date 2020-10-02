use std::cell::{RefCell, RefMut};
use std::convert::TryFrom;
use std::ops::Deref;

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

impl Vanbi {
  pub fn new(selcuha: &ArgMatches<'_>) -> Self {
    Vanbi {
      prina_tarmi: PrinaTarmi::from(selcuha),
      vlacku: RefCell::new(LazniVlacku::try_from(selcuha).unwrap()),
    }
  }

  pub fn vlacku(&self) -> impl Deref<Target = Vlacku> + '_ {
    let judri = self.vlacku.borrow_mut();
    RefMut::map(judri, |x| x.cpacu().expect("Failed to load dictionary"))
  }
}
