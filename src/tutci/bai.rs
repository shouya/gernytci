use crate::kampu::*;

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  selruhe: SelruheStura,
  bai: (Cmavo, Option<Valsi>),
  brivla: (Brivla, Option<Valsi>),
}

#[derive(Clone, Serialize, Debug)]
pub enum SelruheStura {
  Bai(Cmavo),
  Brivla(Brivla),
}

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    let stedu = format!(
      "{} <=> {}",
      self.bai.0.to_string(),
      self.brivla.0.to_string()
    );

    format!(
      "{}\n{}\n{}",
      stedu,
      self
        .bai
        .1
        .clone()
        .and_then(|da| da.smuni)
        .unwrap_or("".into()),
      self
        .brivla
        .1
        .clone()
        .and_then(|da| da.smuni)
        .unwrap_or("".into()),
    )
  }
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<Teryruhe> {
  let selruhe = SelruheStura::zbasu(&value_t!(selruhe, "word", String)?)?;
  let vlacku = vanbi.vlacku()?;

  let teryruhe = match selruhe.clone() {
    SelruheStura::Bai(cmavo) => {
      let valsi = vlacku
        .zvafahi(&cmavo.to_string())
        .ok_or(anyhow!("Unable to find modal"))?;
      let brivla = SelruheStura::ckini_brivla_sisku(&cmavo, &vlacku)
        .ok_or(anyhow!("Unable to find corresponding brivla"))?;
      let brivla_valsi = vlacku.zvafahi(&brivla.to_string());

      Teryruhe {
        selruhe,
        bai: (cmavo.clone(), Some(valsi)),
        brivla: (brivla, brivla_valsi),
      }
    }

    SelruheStura::Brivla(brivla) => {
      let valsi = vlacku
        .zvafahi(&brivla.to_string())
        .ok_or(anyhow!("Unable to find brivla"))?;
      let cmavo = SelruheStura::ckini_cmavrbai_sisku(&brivla, &vlacku)
        .ok_or(anyhow!("Unable to find corresponding brivla"))?;
      let cmavo_valsi = vlacku.zvafahi(&cmavo.to_string());

      Teryruhe {
        selruhe,
        bai: (cmavo, cmavo_valsi),
        brivla: (brivla.clone(), Some(valsi)),
      }
    }
  };

  Ok(teryruhe)
}

impl SelruheStura {
  fn zbasu(selruhe: &str) -> Result<Self> {
    let cmavo = Cmavo::try_from(selruhe).map(SelruheStura::Bai);
    let brivla = Brivla::try_from(selruhe).map(SelruheStura::Brivla);

    cmavo
      .or(brivla)
      .or(Err(anyhow!("input must be either a cmavo or a brivla")))
  }

  fn ckini_brivla_sisku(cmavrbai: &Cmavo, vlacku: &Vlacku) -> Option<Brivla> {
    let ciski = vlacku.zvafahi(&cmavrbai.to_string())?.smuni?;

    let brivla = ciski.split(" modal").next()?.split(" ").last()?;

    Brivla::try_from(brivla).ok()
  }

  fn ckini_cmavrbai_sisku(brivla: &Brivla, vlacku: &Vlacku) -> Option<Cmavo> {
    let brivla = brivla.to_string();
    for valsi in vlacku.iter() {
      if valsi.selmaho != Some("BAI".into()) {
        continue;
      }

      if valsi
        .smuni
        .clone()
        .unwrap_or(String::new())
        .contains(&brivla)
      {
        let cmavo = Cmavo::try_from(valsi.cmene.as_str()).ok()?;
        let fatne_sisku = Self::ckini_brivla_sisku(&cmavo, vlacku)
          .map(|da| da.to_string())
          .unwrap_or(String::new());
        if fatne_sisku == brivla {
          return Some(Cmavo::try_from(valsi.cmene.as_str()).ok()?);
        }
      }
    }

    None
  }
}
