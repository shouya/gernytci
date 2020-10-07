use crate::kampu::*;

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  lei_ciski: Vec<Valsi>,
  lei_lujvo: Vec<Lujvo>,
}

#[derive(Clone, Debug)]
enum Selci {
  Rafsi(Rafsi),
  Sampu(String),
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<Teryruhe> {
  let vlacku = vanbi.vlacku()?;
  let lei_tanru = values_t!(selruhe, "tanru", String).unwrap();
  let lei_valsi: Vec<Valsi> = lei_tanru
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

  let lei_lujvo = ro_cumki(&lei_valsi);
  for jvo in lei_lujvo.iter() {
    println!("{}", jvo.to_string())
  }
  bail!("unimplemented");
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
    .map(|x| x.ro_rafsi())
    .multi_cartesian_product()
    .collect_vec()
    .into_iter()
    .filter_map(|rafpoi| Lujvo::finti(&rafpoi).ok())
    .collect_vec()
}

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    format!("{}", "coi")
  }
}
