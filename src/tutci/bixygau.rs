use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sxd_document::parser as xml_turfahi;
use sxd_xpath;

use crate::kampu::*;
use crate::sidju;
use crate::vlacku::{Valsi, Vlacku};

fn nerbei(krasi_sfaile: &Path, sorcu_sfaile: &Path) -> Result<Vlacku> {
  use sxd_xpath::{Context, Factory, Value};

  let mut vlacku = Vlacku::zbasu(sorcu_sfaile);
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
      _ => bail!("invalid xml"),
    };

  for valsi_tcana in valsi_gunma {
    let tcana = valsi_tcana.element().ok_or(anyhow!("invalid xml"))?;
    let valsi = Valsi::try_from(tcana)?;
    // pu'o zukte .i julne fi loi tolci'o valsi (obsolete words)
    vlacku.selci.push(valsi);
  }

  Ok(vlacku)
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
    let rafsi = Factory::new().build("rafsi")?.unwrap();

    let vanbi = Context::new();
    let facki = |xy_pluta: XPath| {
      xy_pluta
        .evaluate(&vanbi, tcana)
        .map(Value::into_string)
        .map(|da| da.as_str().trim().into())
        .unwrap_or("".into())
    };
    let kunti_cumki = |da: String| if da.is_empty() { None } else { Some(da) };

    let ro_rafsi = match rafsi.evaluate(&vanbi, tcana).unwrap() {
      Value::Nodeset(da) => da
        .document_order()
        .into_iter()
        .map(|x| x.string_value())
        .collect(),
      _ => Vec::new(),
    };

    Ok(Valsi {
      cmene: facki(cmene),
      selmaho: kunti_cumki(facki(selmaho)),
      glosa: kunti_cumki(facki(glosa)),
      smuni: kunti_cumki(facki(smuni)),
      klesi: facki(klesi),
      rafsi: ro_rafsi,
      krasi: facki(krasi),
      pinka: kunti_cumki(facki(pinka)),
    })
  }
}

fn xy_pluta_pe_lo_valsi(pluta: &str) -> sxd_xpath::XPath {
  let fanri = sxd_xpath::Factory::new();
  fanri.build(&format!("string({})", pluta)).unwrap().unwrap()
}

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  klani: usize,
}

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    use colored::*;
    format!("{} words converted.\n", self.klani.to_string().blue())
  }
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<impl Reltai> {
  let krasi = value_t!(selruhe, "from", PathBuf).unwrap();
  let vlacku = nerbei(&krasi, &vanbi.vlacku_sfaile())?;
  vlacku.sorcu()?;

  let klani = vlacku.terkancu();
  Ok(Teryruhe { klani })
}
