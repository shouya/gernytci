use std::convert::{TryFrom, TryInto};

use crate::kampu::*;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub enum Raflei {
  CVC,
  CCV,
  CVhV,
  CVV,
  Brarafsi(Gimlei),
  GismuRafsi(Gimlei),
}

impl TryFrom<&str> for Raflei {
  type Error = anyhow::Error;

  fn try_from(s: &str) -> Result<Self> {
    use Raflei::*;
    match Lerpoi::from(s).sance().as_str() {
      "CVC" => Ok(CVC),
      "CCV" => Ok(CCV),
      "CV'V" => Ok(CVhV),
      "CVV" => Ok(CVV),
      "CVCCV" | "CCVCV" => Gimlei::try_from(s).map(GismuRafsi),
      "CVCC" | "CCVC" => {
        Gimlei::try_from(format!("{}a", s).as_str()).map(Brarafsi)
      }
      _ => bail!("Invalid rafsi"),
    }
  }
}

impl Raflei {
  fn selpormei(&self) -> usize {
    use Raflei::*;
    match self {
      CVC | CCV | CVV => 3,
      CVhV => 4,
      Brarafsi(_) => 4,
      GismuRafsi(_) => 5,
    }
  }

  fn lidne_mapti(lujvo: &str) -> Vec<Raflei> {
    if lujvo.len() < 3 {
      return vec![];
    }

    let mut cumki = vec![];

    for i in 3..=5 {
      match lujvo.get(0..i).and_then(|x| x.try_into().ok()) {
        Some(da) => cumki.push(da),
        _ => {}
      }
    }

    cumki
  }

  pub fn xu_tamsmi_cyvyvy(&self) -> bool {
    use Raflei::*;
    *self == CVV || *self == CVhV
  }

  pub fn xu_sampu(&self) -> bool {
    use Raflei::*;
    match self {
      GismuRafsi(_) | Brarafsi(_) => false,
      _ => true,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Rafsi {
  pub rafsi: String,
  pub terjonlehu: Option<char>,
}

impl ToString for Rafsi {
  fn to_string(&self) -> String {
    let terjonlehu = match self.terjonlehu {
      Some(c) => format!("{}", c),
      _ => "".to_string(),
    };

    format!("{}{}", self.rafsi, terjonlehu)
  }
}

impl Rafsi {
  pub fn klesi(&self) -> Raflei {
    self.rafsi.as_str().try_into().unwrap()
  }

  pub fn selpormei(&self) -> usize {
    self.klesi().selpormei() + self.terjonlehu.map(|_| 1).unwrap_or(0)
  }

  // spuda fi loi romei cumki
  pub fn genturfahi_bavlahi(lujvo: &str) -> Vec<Self> {
    let mut cumki = Vec::new();

    for raflei in Raflei::lidne_mapti(lujvo) {
      let selpormei = raflei.selpormei();

      let lerpoi = lujvo[0..selpormei].to_string();

      match lujvo[raflei.selpormei()..].chars().next() {
        Some(lerfu) if Rafsi::jvasahe(&lerpoi, Some(lerfu)) => {
          // pamoi cumki: lo sevzi ce'o pa terjonle'u
          cumki.push(Rafsi {
            rafsi: lerpoi.clone(),
            terjonlehu: Some(lerfu),
          });
          cumki.push(Rafsi {
            rafsi: lerpoi,
            terjonlehu: None,
          })
        }
        Some(_lerfu) if Rafsi::jvasahe(&lerpoi, None) => {
          // remoi cumki: lo sevzi
          cumki.push(Rafsi {
            rafsi: lerpoi,
            terjonlehu: None,
          })
        }
        None if Rafsi::jvasahe(&lerpoi, None) => {
          // cimoi cumki: rebla be lo lerpoi
          cumki.push(Rafsi {
            rafsi: lujvo[0..selpormei].to_string(),
            terjonlehu: None,
          });
        }
        _ => (),
      }
    }

    cumki.dedup();
    cumki
  }

  pub fn jvasahe(lerpoi: &str, terjonlehu: Option<char>) -> bool {
    if lerpoi.len() < 3 || lerpoi.len() > 5 {
      return false;
    }

    let raflei = match Raflei::try_from(lerpoi) {
      Ok(klesi) => klesi,
      _ => return false,
    };

    Self::lerpoi_jvasahe(lerpoi, raflei)
      && Self::terjonlehu_jvasahe(raflei, terjonlehu)
  }

  pub fn lerpoi_jvasahe(lerpoi: &str, raflei: Raflei) -> bool {
    use Raflei::*;
    let zunsna_sarxe = |xoxipa, xoxire| {
      let pa = lerpoi.chars().nth(xoxipa).unwrap();
      let re = lerpoi.chars().nth(xoxire).unwrap();
      Lerfu::zunsna_sarxe(pa, re)
    };

    match raflei {
      CVC | CVhV | CVV => true,
      CCV if zunsna_sarxe(0, 1) => true,
      GismuRafsi(gimlei) => Gismu::lerpoi_jvasahe(lerpoi, gimlei),
      Brarafsi(gimlei) => Gismu::lerpoi_jvasahe(lerpoi, gimlei),
      _ => false,
    }
  }

  pub fn terjonlehu_jvasahe(klesi: Raflei, terjonlehu: Option<char>) -> bool {
    use Raflei::*;

    match (klesi, terjonlehu) {
      (ky, None) if ky.xu_sampu() => return true,
      (ky, Some(_)) if ky.xu_sampu() => (),
      (GismuRafsi(_), None) => return true,
      (Brarafsi(_), Some('y')) => return true,
      _ => return false,
    }

    let terjonlehu = terjonlehu.unwrap();

    match terjonlehu {
      'y' | 'r' | 'n' => (),
      _ => return false,
    }

    if terjonlehu == 'y' && klesi == CVC {
      true
    } else if terjonlehu == 'r' && klesi.xu_tamsmi_cyvyvy() {
      true
    } else if terjonlehu == 'n' && klesi.xu_tamsmi_cyvyvy() {
      true
    } else {
      false
    }
  }

  pub fn vlaste_sisku<'a>(&self, vlacku: &'a Vlacku) -> Option<&'a Valsi> {
    use Raflei::*;

    for valsi in vlacku.iter() {
      let found = match self.klesi() {
        Brarafsi(_) => valsi.cmene.starts_with(&self.rafsi[0..4]),
        GismuRafsi(_) => valsi.cmene == self.rafsi,
        _ => valsi.rafsi.contains(&self.rafsi),
      };

      if found {
        return Some(valsi);
      }
    }

    None
  }
}
