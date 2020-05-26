use crate::kampu::*;
use crate::vlacku::{Valsi, Vlacku};

use serde::Serialize;
use serde_json;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tanru", about = "Split lujvo into tanru")]
pub struct Tergalfi {
  #[structopt(name = "word")]
  lujvo: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  rafsi: Vec<Rafsi>,
  tanru: Vec<Option<Valsi>>,
}

impl crate::TciTeryruhe for Teryruhe {
  fn termontai_lo_vlamei(&self) {
    use colored::*;

    let rafsi: String = self
      .rafsi
      .iter()
      .map(|da| da.to_string())
      .map(|da| da.blue().to_string())
      .collect::<Vec<_>>()
      .join("/");

    let tanru: String = self
      .tanru
      .iter()
      .zip(self.rafsi.iter())
      .map(|(da, de)| match da {
        Some(Valsi { cmene, .. }) => cmene.clone(),
        None => de.to_string(),
      })
      .map(|da| da.green().to_string())
      .collect::<Vec<_>>()
      .join(" ");

    let glosa: String = self
      .tanru
      .iter()
      .map(|da| match da {
        Some(Valsi { glosa, .. }) => glosa.clone().unwrap_or("...".into()),
        None => "...".into(),
      })
      .map(|da| da.yellow().to_string())
      .collect::<Vec<_>>()
      .join(" + ");

    println!("{}", rafsi);
    println!("{}", tanru);
    println!("{}", glosa);
  }

  fn termontai_lahe_jeison(&self) {
    println!("{}", serde_json::to_string(self).unwrap())
  }
}

pub fn tanru(tergaf: &crate::Tergalfi, vlacku: &Vlacku) -> Result<Teryruhe> {
  use crate::Minde::Tanru;
  let tamsmi_tergaf = match &tergaf.minde {
    Tanru(da) => da,
    _ => unreachable!(),
  };

  let rafsi = match katna_lujvo(tamsmi_tergaf.lujvo.as_str()) {
    Some(raf) => raf,
    None => vec![],
  };

  let tanru = rafsi.iter().map(|raf| sisku_tanru(raf, vlacku)).collect();

  Ok(Teryruhe { rafsi, tanru })
}

fn sisku_tanru(rafsi: &Rafsi, vlacku: &Vlacku) -> Option<Valsi> {
  use Raflei::*;

  for valsi in &vlacku.sorcu {
    let found = match rafsi.klesi {
      Brarafsi => valsi.cmene.starts_with(&rafsi.rafsi[0..4]),
      Gismu(_) => valsi.cmene == rafsi.rafsi,
      _ => valsi.rafsi.contains(&rafsi.rafsi),
    };

    if found {
      return Some(valsi.clone());
    }
  }

  None
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[allow(dead_code)]
enum Raflei {
  CVC,
  CCV,
  CVhV,
  CVV,
  Brarafsi,
  Gismu(Gimlei),
}

impl Raflei {
  fn len(&self) -> usize {
    use Raflei::*;
    match self {
      CVC | CCV | CVV => 3,
      CVhV => 4,
      Brarafsi => 4,
      Gismu(_) => 5,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[allow(dead_code)]
enum Gimlei {
  CVCCV,
  CCVCV,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
struct Rafsi {
  klesi: Raflei,
  rafsi: String,
}

impl ToString for Rafsi {
  fn to_string(&self) -> String {
    self.rafsi.clone()
  }
}

fn katna_lujvo(lujvo: &str) -> Option<Vec<Rafsi>> {
  match lujvo.len() {
    0 => return Some(vec![]),
    1 | 2 => return None,
    _ => (),
  }

  if lujvo.len() == 5 {
    return jorne(gismu_zvafahi(lujvo), &lujvo[5..]);
  }
  if lujvo.len() == 3 || lujvo.len() == 4 {
    if let Some(rafsi) = rafsi_zvafahi(lujvo) {
      let len = rafsi.klesi.len();
      return jorne(Some(rafsi), &lujvo[len..]);
    } else {
      return None;
    }
  }

  // lu [gerny]tci li'u mu'a
  if let Some(rafsi) = brarafsi_zvafahi(lujvo) {
    return jorne(Some(rafsi), &lujvo[5..]);
  }

  // lu [loby]bau li'u mu'a
  if let Some(rafsi) = rafsi_ceho_terjonlehu_zvafahi(lujvo) {
    return jorne(Some(rafsi), &lujvo[4..]);
  }

  // lu [jbo]bau li'u mu'a
  if let Some(rafsi) = rafsi_zvafahi(lujvo) {
    let len = rafsi.klesi.len();
    return jorne(Some(rafsi), &lujvo[len..]);
  }

  return None;
}

fn gismu_zvafahi(lujvo: &str) -> Option<Rafsi> {
  gismu_klesi(&lujvo[0..5]).map(|gimlei| Rafsi {
    klesi: Raflei::Gismu(gimlei),
    rafsi: (&lujvo[0..5]).into(),
  })
}

fn rafsi_zvafahi(lujvo: &str) -> Option<Rafsi> {
  if let Some(klesi) = rafsi_klesi(&lujvo[0..3]) {
    return Some(Rafsi {
      klesi,
      rafsi: (&lujvo[0..3]).into(),
    });
  }

  if let Some(klesi) = rafsi_klesi(&lujvo[0..4]) {
    return Some(Rafsi {
      klesi,
      rafsi: (&lujvo[0..4]).into(),
    });
  }

  return None;
}

fn brarafsi_zvafahi(lujvo: &str) -> Option<Rafsi> {
  if lujvo.chars().nth(4).unwrap() != 'y' {
    return None;
  }

  gismu_klesi(&format!("{}{}", &lujvo[0..4], 'a')).map(|_| {
    return Rafsi {
      klesi: Raflei::Brarafsi,
      rafsi: (&lujvo[0..5]).into(),
    };
  })
}

fn rafsi_ceho_terjonlehu_zvafahi(lujvo: &str) -> Option<Rafsi> {
  use Raflei::*;

  let terjonlehu = lujvo.chars().nth(3).unwrap();

  match terjonlehu {
    'r' | 'y' | 'n' => (),
    _ => return None,
  };

  let seltau = &lujvo[0..3];
  let tertau = &lujvo[4..7];

  let seltau_klesi = rafsi_klesi(seltau)?;
  let tertau_klesi = rafsi_klesi(tertau)?;

  if terjonlehu == 'y' {
    let pa = seltau.chars().nth(2).unwrap();
    let re = tertau.chars().nth(0).unwrap();
    if seltau_klesi == CVC && !CURMI_ZUNSNA_REMEI.contains(&(pa, re)) {
      return Some(Rafsi {
        klesi: seltau_klesi,
        rafsi: (&lujvo[0..4]).into(),
      });
    } else {
      return None;
    }
  }

  if terjonlehu == 'n' && tertau.chars().nth(0).unwrap() != 'r' {
    return None;
  }

  // lo tanru poi zilzba re pagbu
  if lujvo.len() == 7 {
    if seltau_klesi == CVV && tertau_klesi != CCV {
      return Some(Rafsi {
        klesi: seltau_klesi,
        rafsi: (&lujvo[0..4]).into(),
      });
    } else {
      return None;
    }
  }

  if seltau_klesi == CVV {
    return Some(Rafsi {
      klesi: seltau_klesi,
      rafsi: (&lujvo[0..4]).into(),
    });
  }

  return None;
}

const CURMI_ZUNSNA_REMEI: &'static [(char, char)] = &[
  ('p', 'l'),
  ('p', 'r'),
  ('f', 'l'),
  ('f', 'r'),
  ('b', 'l'),
  ('b', 'r'),
  ('v', 'l'),
  ('v', 'r'),
  ('c', 'p'),
  ('c', 'f'),
  ('c', 't'),
  ('c', 'k'),
  ('c', 'm'),
  ('c', 'n'),
  ('c', 'l'),
  ('c', 'r'),
  ('j', 'b'),
  ('j', 'v'),
  ('j', 'd'),
  ('j', 'g'),
  ('j', 'm'),
  ('s', 'p'),
  ('s', 'f'),
  ('s', 't'),
  ('s', 'k'),
  ('s', 'm'),
  ('s', 'n'),
  ('s', 'l'),
  ('s', 'r'),
  ('z', 'b'),
  ('z', 'v'),
  ('z', 'd'),
  ('z', 'g'),
  ('z', 'm'),
  ('t', 'c'),
  ('t', 'r'),
  ('t', 's'),
  ('k', 'l'),
  ('k', 'r'),
  ('d', 'j'),
  ('d', 'r'),
  ('d', 'z'),
  ('g', 'l'),
  ('g', 'r'),
  ('m', 'l'),
  ('m', 'r'),
  ('x', 'l'),
  ('x', 'r'),
];

fn jorne(rafsi: Option<Rafsi>, selyliha: &str) -> Option<Vec<Rafsi>> {
  if rafsi.is_none() {
    return None;
  }

  if let Some(mut selyliha_rafsi) = katna_lujvo(selyliha) {
    selyliha_rafsi.insert(0, rafsi.unwrap());
    Some(selyliha_rafsi)
  } else {
    None
  }
}

fn gismu_klesi(gismu: &str) -> Option<Gimlei> {
  use Gimlei::*;
  if gismu.len() != 5 {
    return None;
  }

  match lerfu_sanse(gismu).as_str() {
    "CVCCV" => Some(CVCCV),
    "CCVCV" => Some(CCVCV),
    _ => None,
  }
}

fn rafsi_klesi(rafsi: &str) -> Option<Raflei> {
  use Raflei::*;

  if rafsi.len() != 3 && rafsi.len() != 4 {
    return None;
  }

  match lerfu_sanse(rafsi).as_str() {
    "CVC" => Some(CVC),
    "CCV" => Some(CCV),
    "CV'V" => Some(CVhV),
    "CVV" => Some(CVV),
    _ => None,
  }
}

fn lerfu_sanse(valsi: &str) -> String {
  let mut teryruhe = String::new();
  for lerfu in valsi.to_lowercase().chars() {
    if let Some(_) = "aeiou".find(lerfu) {
      teryruhe.push('V');
      continue;
    }
    if lerfu.is_ascii_alphabetic() {
      teryruhe.push('C');
      continue;
    }
    teryruhe.push(lerfu);
  }
  teryruhe
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cipra_katna_lujvo() {
    xusra_katna("saicli", "sai/cli");
    xusra_katna("saiclire", "sai/clire");
    xusra_katna("sanmycli", "sanm/cli");
    xusra_katna("sanmycilre", "sanm/cilre");
    xusra_katna("zvaju'o", "zva/ju'o");
    xusra_katna("ju'ozva", "ju'o/zva");
    xusra_katna("ju'ozva", "ju'o/zva");
    xusra_katna("cmeterge'a", "cme/ter/ge'a");

    assert!(katna_lujvo("saiycli").is_none());
    assert!(katna_lujvo("saircli").is_none());
    assert!(katna_lujvo("saincli").is_none());
  }

  fn xusra_katna(lujvo: &str, lei_rafsi: &str) {
    if let Some(teryruhe) = katna_lujvo(lujvo) {
      let teryruhe_rafsi: Vec<_> =
        teryruhe.into_iter().map(|x| x.rafsi).collect();
      assert_eq!(teryruhe_rafsi.as_slice().join("/"), lei_rafsi);
    } else {
      assert!(false)
    }
  }
}
