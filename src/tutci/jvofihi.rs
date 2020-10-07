use crate::kampu::*;

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  lei_terspu: Vec<String>,
  lei_tanru: Vec<TanruSelci>,
  lei_lujvo: Vec<LujvoRutni>,
}

#[derive(Clone, Debug)]
enum TerspuSelci {
  Rafsi(Rafsi),
  Sampu(String),
}

#[derive(Clone, Serialize, Debug)]
pub enum TanruSelci {
  Valsi(Valsi),
  NarjuhoGismu(String),
}

#[derive(Clone, Serialize, Debug)]
pub struct LujvoRutni {
  lujvo: Lujvo,
  vamji: usize,
  valsi: Option<Valsi>,
}

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    let valsi_lerpoi = self.lei_tanru.iter().map(TanruSelci::cmene).join(" ");
    let glosa_lerpoi = self
      .lei_tanru
      .iter()
      .map(|da| da.glosa().unwrap_or("..."))
      .join(" + ");

    let rutni_lerpoi = |rutni: &LujvoRutni| match &rutni.valsi {
      Some(valsi) => format!(
        "{} ({}) - {}",
        rutni.lujvo.to_string(),
        rutni.vamji,
        valsi.tordu_ciski(50).unwrap_or("...".to_string())
      ),
      None => format!("{} ({})", rutni.lujvo.to_string(), rutni.vamji,),
    };

    let lei_lujvo_vlalihi = self.lei_lujvo.iter().map(rutni_lerpoi).join("\n");
    format!(
      "{}\n{}\n---\n{}\n",
      valsi_lerpoi, glosa_lerpoi, lei_lujvo_vlalihi,
    )
  }
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<Teryruhe> {
  let vlacku = vanbi.vlacku()?;
  let lei_terspu: Vec<String> = values_t!(selruhe, "tanru", String)
    .unwrap()
    .into_iter()
    .flat_map(|x| x.split(" ").map(String::from).collect_vec())
    .collect_vec();
  let lei_tanru: Vec<TanruSelci> = lei_terspu
    .iter()
    .map(|da| TerspuSelci::turfahi(da))
    .collect::<Result<Vec<_>>>()?
    .into_iter()
    .flatten()
    .map(|selci| selci.tanru_selci(&vlacku))
    .collect::<Result<_>>()?;

  let lei_lujvo = ro_cumki(&lei_tanru)
    .into_iter()
    .map(|lujvo| LujvoRutni::zbasu(lujvo, &vlacku))
    .sorted_by_key(|lujvo_rutni| lujvo_rutni.vamji)
    .collect_vec();

  Ok(Teryruhe {
    lei_lujvo,
    lei_terspu,
    lei_tanru,
  })
}

impl TerspuSelci {
  fn turfahi(terspu: &str) -> Result<Vec<Self>> {
    if Gimlei::try_from(terspu).is_ok() {
      return Ok(vec![TerspuSelci::Sampu(terspu.to_string())]);
    }

    let lei_lujvo = Lujvo::try_from(terspu).map(|lujvo| {
      lujvo
        .iter()
        .map(|rafsi| TerspuSelci::Rafsi(rafsi.clone()))
        .collect_vec()
    });

    let rafsi = Rafsi::try_from(terspu).map(|da| vec![TerspuSelci::Rafsi(da)]);

    let cmavo = Ok(vec![TerspuSelci::Sampu(terspu.to_string())]);

    lei_lujvo.or(rafsi).or(cmavo)
  }

  fn cmene(&self) -> &str {
    match self {
      TerspuSelci::Rafsi(rafsi) => &rafsi.rafsi,
      TerspuSelci::Sampu(cmene) => &cmene,
    }
  }

  fn gismu_cmene(&self) -> Option<String> {
    match self {
      TerspuSelci::Rafsi(rafsi) => match rafsi.klesi() {
        Raflei::Brarafsi(_) => Some(rafsi.to_string()),
        Raflei::GismuRafsi(_) => Some(rafsi.to_string()),
        _ => None,
      },
      TerspuSelci::Sampu(cmene) => {
        Gimlei::try_from(cmene.as_str()).ok().map(|_| cmene.into())
      }
    }
  }

  fn tanru_selci(&self, vlacku: &Vlacku) -> Result<TanruSelci> {
    let kahe_valsi = match self {
      TerspuSelci::Rafsi(rafsi) => rafsi.vlaste_sisku(vlacku).map(Clone::clone),
      TerspuSelci::Sampu(cmene) => vlacku.zvafahi(cmene),
    };

    match kahe_valsi {
      Some(valsi) => Ok(TanruSelci::Valsi(valsi)),
      None => self
        .gismu_cmene()
        .ok_or(anyhow!("No dictionary word for {}", self.cmene()))
        .map(|da| TanruSelci::NarjuhoGismu(da)),
    }
  }
}

fn ro_cumki(lei_selci: &Vec<TanruSelci>) -> Vec<Lujvo> {
  lei_selci
    .iter()
    .map(|tanru_selci| tanru_selci.rafsi_mei())
    .multi_cartesian_product()
    .collect_vec()
    .into_iter()
    .filter_map(|rafpoi| Lujvo::finti(&rafpoi).ok())
    .collect_vec()
}

impl TanruSelci {
  fn rafsi_mei(&self) -> Vec<String> {
    match self {
      TanruSelci::Valsi(valsi) => valsi.rafsi_mei(),
      TanruSelci::NarjuhoGismu(gismu) => {
        vec![gismu[..4].to_string(), gismu.clone()]
      }
    }
  }

  fn cmene(&self) -> &str {
    match self {
      TanruSelci::Valsi(valsi) => &valsi.cmene,
      TanruSelci::NarjuhoGismu(gismu) => &gismu,
    }
  }

  fn glosa(&self) -> Option<&str> {
    match self {
      TanruSelci::Valsi(valsi) => valsi.glosa.as_ref().map(String::as_str),
      TanruSelci::NarjuhoGismu(_gismu) => None,
    }
  }
}

impl LujvoRutni {
  fn zbasu(lujvo: Lujvo, vlacku: &Vlacku) -> Self {
    let valsi = vlacku.zvafahi(&lujvo.to_string());
    let vamji = lujvo.jvovahi();

    Self {
      valsi,
      lujvo,
      vamji,
    }
  }
}
