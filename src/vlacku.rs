use std::convert::TryFrom;
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
    let xml_xadni = xml_turfahi::parse(&sfaile_xadni)?;
    let uencu = xml_xadni.as_document();

    let valsi_xy_pluta = Factory::new()
      .build(concat!(
        "//dictionary",
        r#"/direction[@from="lojban"][@to="English"]"#,
        "/valsi"
      ))?
      .unwrap();

    let valsi_gunma =
      match valsi_xy_pluta.evaluate(&Context::new(), uencu.root())? {
        Value::Nodeset(da) => da,
        _ => return Err(anyhow!("invalid xml")),
      };

    let mut sorcu = Vec::new();
    for valsi_tcana in valsi_gunma {
      let tcana = valsi_tcana.element().ok_or(anyhow!("invalid xml"))?;
      let valsi = Valsi::try_from(tcana)?;
      // pu'o zukte .i julne fi loi tolci'o valsi (obsolete words)
      sorcu.push(valsi)
    }

    Ok(Vlacku { sorcu })
  }
}

impl<'d> TryFrom<sxd_document::dom::Element<'d>> for Valsi {
  type Error = Error;
  fn try_from(tcana: sxd_document::dom::Element<'d>) -> Result<Self> {
    use sxd_xpath::{Context, Factory, Value, XPath};

    let cmene = xy_pluta_pe_lo_valsi("@word");
    let klesi = xy_pluta_pe_lo_valsi("@type");
    let smuni = xy_pluta_pe_lo_valsi("definition");
    let selmaho = xy_pluta_pe_lo_valsi("selmaho");
    let glosa = xy_pluta_pe_lo_valsi("glossword[1]/@word");
    let krasi = xy_pluta_pe_lo_valsi("user/username");
    let pinka = xy_pluta_pe_lo_valsi("notes");
    let rafsi = Factory::new().build("/valsi/rafsi/text()")?.unwrap();

    let vanbi = Context::new();
    let facki = |xy_pluta: XPath| {
      xy_pluta
        .evaluate(&vanbi, tcana)
        .map(Value::into_string)
        .unwrap_or("".into())
    };
    let kunti_cumki = |da: String| if da.is_empty() { None } else { Some(da) };

    let _ro_rafsi = dbg!(rafsi.evaluate(&vanbi, tcana));

    Ok(Valsi {
      cmene: facki(cmene),
      selmaho: facki(selmaho),
      glosa: kunti_cumki(facki(glosa)),
      smuni: kunti_cumki(facki(smuni)),
      klesi: facki(klesi),
      rafsi: Vec::new(),
      krasi: facki(krasi),
      pinka: kunti_cumki(facki(pinka)),
    })
  }
}

fn xy_pluta_pe_lo_valsi(pluta: &str) -> sxd_xpath::XPath {
  let fanri = sxd_xpath::Factory::new();
  fanri.build(&format!("string(/vlasi/{})", pluta)).unwrap().unwrap()
}
