use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::vlacku::Vlacku;

pub struct Vanbi<'a> {
  tcekau_selcuha: &'a ArgMatches<'a>,
  vlacku: RefCell<Option<Vlacku>>,
}

impl<'a> Vanbi<'a> {
  pub fn new(selcuha: &'a ArgMatches<'a>) -> Self {
    Vanbi {
      tcekau_selcuha: selcuha,
      vlacku: RefCell::new(None as Option<Vlacku>),
    }
  }

  pub fn cpacu_vlacku(&self) -> impl Deref<Target = Vlacku> + '_ {
    match &mut *self.vlacku.borrow_mut() {
      Some(_vlacku) => (),
      judri => {
        let selcuha = self.tcekau_selcuha;
        let sfaile = PathBuf::from(selcuha.value_of("dictionary").unwrap());
        let mut vlacku =
          Vlacku::tolsorcu(&sfaile).expect("Failed to load dictionary");

        if selcuha.is_present("official-only") {
          vlacku.catni_poho()
        }

        judri.replace(vlacku);
      }
    }

    Ref::map(self.vlacku.borrow(), |x| x.as_ref().unwrap())
  }
}
