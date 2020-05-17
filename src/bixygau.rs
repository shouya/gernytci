use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use serde::Serialize;
use structopt::StructOpt;
use sxd_document::parser as xml_turfahi;
use sxd_xpath;

use crate::kampu::*;
use crate::sidju;
use crate::vlacku::{Valsi, Vlacku};

fn nerbei(vlacku: &mut Vlacku, krasi_sfaile: &Path) -> Result<()> {
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

  for valsi_tcana in valsi_gunma {
    let tcana = valsi_tcana.element().ok_or(anyhow!("invalid xml"))?;
    let valsi = Valsi::try_from(tcana)?;
    // pu'o zukte .i julne fi loi tolci'o valsi (obsolete words)
    vlacku.jmina(valsi)
  }

  Ok(())
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
    let rafsi = Factory::new().build("rafsi/text()")?.unwrap();

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

#[derive(StructOpt)]
#[structopt(
  name = "convert",
  about = "Convert xml dump from jbovlaste (<dict> needs to be writable)"
)]
pub struct Tergalfi {
  #[structopt(
    name = "from",
    short,
    long,
    parse(from_os_str),
    about = "location of XML dump file (download from http://jbovlaste.lojban.org/export/xml-export.html?lang=en)"
  )]
  krasi: PathBuf,
}

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  klani: usize,
}

impl crate::TciTeryruhe for Teryruhe {
  fn termontai_lo_vlamei(&self) {
    use colored::*;
    println!("{} words converted.", self.klani.to_string().blue());
  }

  fn termontai_lahe_jeison(&self) {
    println!("{}", serde_json::to_string(self).unwrap())
  }
}

pub fn bixygau(
  tergaf: &crate::Tergalfi,
  vlacku: &mut Vlacku,
) -> Result<Teryruhe> {
  use crate::Minde::Bixygau;
  let bixygau_tergaf = match &tergaf.minde {
    Bixygau(da) => da,
    _ => unreachable!(),
  };

  nerbei(vlacku, &bixygau_tergaf.krasi)?;
  vlacku.sorcu()?;

  let klani = vlacku.sorcu.len();
  Ok(Teryruhe { klani })
}
