use crate::kampu::*;

type Jvovahi = usize;

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  lei_selyspu: Vec<String>,
  lei_tanru: Vec<Valsi>,
  lei_lujvo: Vec<(Jvovahi, Lujvo)>,
}

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    let valsi_lerpoi = self.lei_tanru.iter().map(|x| x.cmene.clone()).join(" ");
    let glosa_lerpoi = self
      .lei_tanru
      .iter()
      .map(|x| x.glosa.clone().unwrap_or("...".into()))
      .join(" + ");
    let lei_lujvo_vlalihi = self
      .lei_lujvo
      .iter()
      .map(|(vamji, lujvo)| format!("- {} ({})", lujvo.to_string(), vamji))
      .join("\n");
    format!(
      "{}\n{}\n{}\n",
      valsi_lerpoi, glosa_lerpoi, lei_lujvo_vlalihi,
    )
  }
}

#[derive(Clone, Debug)]
enum Selci {
  Rafsi(Rafsi),
  Sampu(String),
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<Teryruhe> {
  let vlacku = vanbi.vlacku()?;
  let lei_selyspu: Vec<String> = values_t!(selruhe, "tanru", String)
    .unwrap()
    .into_iter()
    .flat_map(|x| x.split(" ").map(String::from).collect_vec())
    .collect_vec();
  let lei_tanru: Vec<Valsi> = lei_selyspu
    .iter()
    .flat_map(|da| match Lujvo::genturfahi(da).pop() {
      Some(lujvo) => lujvo
        .iter()
        .map(|rafsi| Selci::Rafsi(rafsi.clone()))
        .collect(),
      _ => vec![Selci::Sampu(da.clone())],
    })
    .map(|selci| {
      &selci;
      selci
        .zvafahi_valsi(&vlacku)
        .ok_or_else(|| anyhow!("Fail to find corresponding word"))
    })
    .collect::<Result<_>>()?;

  let lei_lujvo = ro_cumki(&lei_tanru)
    .into_iter()
    .map(|lujvo| (lujvo.jvovahi(), lujvo))
    .sorted_by_key(|(vamji, _)| *vamji)
    .collect_vec();

  Ok(Teryruhe {
    lei_lujvo,
    lei_selyspu,
    lei_tanru,
  })
}

impl Selci {
  fn zvafahi_valsi(&self, vlacku: &Vlacku) -> Option<Valsi> {
    let kahe_valsi = match self {
      Selci::Rafsi(rafsi) => rafsi.vlaste_sisku(vlacku).map(Clone::clone),
      Selci::Sampu(cmene) => vlacku.zvafahi(cmene),
    };

    match kahe_valsi {
      None => None,
      Some(valsi) => {
        if valsi.rafsi.is_empty() {
          None
        } else {
          Some(valsi)
        }
      }
    }
  }
}

fn ro_cumki(lei_selci: &Vec<Valsi>) -> Vec<Lujvo> {
  lei_selci
    .iter()
    .map(|valsi| valsi.rafsi_mei())
    .multi_cartesian_product()
    .collect_vec()
    .into_iter()
    .filter_map(|rafpoi| Lujvo::finti(&rafpoi).ok())
    .collect_vec()
}
