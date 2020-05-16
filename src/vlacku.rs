use std::path::Path;

use crate::kampu::*;
use crate::sidju;

use serde::{Deserialize, Serialize};
use serde_json;
use sxd_document::parser as xml_turfahi;
use sxd_xpath;

type Selmaho = String;
type Rafsi = String;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Valsi {
  pub cmene: String,
  pub klesi: String,
  pub selmaho: Selmaho,
  pub glosa: Option<String>,
  pub smuni: Option<String>,
  pub rafsi: Vec<Rafsi>,
  pub krasi: String,
  pub pinka: Option<String>,
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

  pub fn jmina(&mut self, valsi: Valsi) {
    self.sorcu.push(valsi)
  }

  // pu'o zukte
  pub fn nerbei(krasi_sfaile: &Path) -> Result<Self> {
    use sxd_xpath::{Context, Factory, Value};
    let sfaile_xadni = sidju::tolsorcu_sfaile(krasi_sfaile)?;
    let uencu = xml_turfahi::parse(&sfaile_xadni)?.as_document();

    let fanri = Factory::new();

    let valsi_pluta = fanri
      .build(concat!(
        "//dictionary",
        r#"/direction[@from="lojban"][@to="English"]"#,
        "/valsi"
      ))?
      .unwrap();

    let mut sorcu = Vec::new();

    let valsi_gunma = match valsi_pluta.evaluate(&vanbi, uencu.root())? {
      Value::Nodeset(da) => da,
      _ => return Err(anyhow!("invalid export structure")),
    };

    for valsi_tcana in valsi_gunma {
      if let Some(valsi) = binxo_pa_valsi(valsi_tcana) {
        // pu'o zukte .i julne fi loi tolci'o valsi (obsolete words)
        sorcu.push(Valsi::try_from(valsi));
      }
    }

    Ok(Vlacku { sorcu })
  }
}

impl<'d> TryFrom<sxd_document::dom::Element<'d>> for Valsi {
  type Error = Error;
  fn try_from(valsi_tcana: sxd_document::dom::Element<'d>) -> Result<Self> {
    use sxd_xpath::{Context, Factory, Value};

    lazy_static! {
      static ref fanri = Factory::new();

      static ref pluta_zbasu = |pluta| {
        fanri.build(format!("string(/vlasi/{})", pluta))?.unwrap();
      }

      static ref cmene = pluta_zbasu("@word");
      static ref klesi = pluta_zbasu("@type");
      static ref selmaho = pluta_zbasu("selmaho");
      static ref glosa = pluta_zbasu("glossword[1]/@word");
      static ref krasi = pluta_zbasu("user/username");
      static ref pinka = pluta_zbasu("notes");
      static ref rafsi = fanri.build("/valsi/rafsi/text()")?.unwrap();
    }

    let vanbi = Context::new();
    let valsi = valsi_tcana.element().ok_or(anyhow!("invalid DOM"))?;
    let facki = |pluta| pluta.evaluate(&vanbi, valsi).map(Value::into_string());
    let kunti_cumki = |da| if da.is_empty() { None } else { Some(da) };

    let ro_rafsi = dbg!(rafsi.evaluate(&vanbi, valsi));

    Ok(Valsi {
      cmene: facki(cmene),
      selmaho: facki(selmaho),
      glosa: kunti_cumki(facki(glosa)),
      smuni: kunti_cumki(facki(smuni)),
      rafsi: Vec::new(),
      krasi: facki(krasi),
      pinka: faski(pinka),
    })
  }
}
