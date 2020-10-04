use crate::kampu::*;

use crate::gerna::lerfu::lerpoi_sanse;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[allow(dead_code)]
pub enum Raflei {
  CVC,
  CCV,
  CVhV,
  CVV,
  Brarafsi,
  Gismu(Gimlei),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[allow(dead_code)]
pub enum Gimlei {
  CVCCV,
  CCVCV,
}

impl Raflei {
  fn selpormei(&self) -> usize {
    use Raflei::*;
    match self {
      CVC | CCV | CVV => 3,
      CVhV => 4,
      Brarafsi => 4,
      Gismu(_) => 5,
    }
  }

  fn lidne_mapti(lujvo: &str) -> Vec<Raflei> {
    if lujvo.len() < 3 {
      return vec![];
    }

    let mut cumki = vec![];

    for i in 3..=5 {
      match lujvo.get(0..i).and_then(|x| Self::mapti(x)) {
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

  fn mapti(rafsi: &str) -> Option<Raflei> {
    use Gimlei::*;
    use Raflei::*;

    if rafsi.len() < 3 || rafsi.len() > 5 {
      return None;
    }

    match lerpoi_sanse(rafsi).as_str() {
      "CVC" => Some(CVC),
      "CCV" => Some(CCV),
      "CV'V" => Some(CVhV),
      "CVV" => Some(CVV),
      "CVCCV" => Some(Gismu(CVCCV)),
      "CCVCV" => Some(Gismu(CCVCV)),
      "CVCC" => Some(Brarafsi),
      "CCVC" => Some(Brarafsi),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Rafsi {
  pub klesi: Raflei,
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
  pub fn selpormei(&self) -> usize {
    self.klesi.selpormei() + self.terjonlehu.map(|_| 1).unwrap_or(0)
  }

  // spuda fi loi romei cumki
  pub fn genturfahi_bavlahi(lujvo: &str) -> Vec<Self> {
    let mut cumki = Vec::new();

    for raflei in Raflei::lidne_mapti(lujvo) {
      // pamoi cumki: lo sevzi
      cumki.push(Rafsi {
        klesi: raflei.clone(),
        rafsi: lujvo[0..raflei.selpormei()].to_string(),
        terjonlehu: None,
      });

      // remoi cumki: lo sevzi ce'o pa terjonle'u
      if let Some(lerfu) = lujvo[raflei.selpormei()..].chars().next() {
        cumki.push(Rafsi {
          klesi: raflei.clone(),
          rafsi: lujvo[0..raflei.selpormei()].to_string(),
          terjonlehu: Some(lerfu),
        });
      }
    }

    cumki.dedup();
    cumki.into_iter().filter(|x| x.jvasahe()).collect()
  }

  pub fn jvasahe(&self) -> bool {
    use Raflei::*;

    let jvasahe_pe_le_terjonlehu = match self.klesi {
      Gismu(_) => self.terjonlehu == None,
      Brarafsi => self.terjonlehu == Some('y'),
      CVC | CCV | CVhV | CVV if self.terjonlehu == None => true,
      CVC | CCV | CVhV | CVV => self.rafsi_terjonlehu_jvasahe(),
    };

    jvasahe_pe_le_terjonlehu
  }

  pub fn rafsi_terjonlehu_jvasahe(&self) -> bool {
    use Raflei::*;
    let klesi = match &self.klesi {
      Gismu(_) | Brarafsi => panic!("Invalid input"),
      klesi => klesi.clone(),
    };

    let terjonlehu = match self.terjonlehu {
      None => return true,
      Some('y') | Some('r') | Some('n') => self.terjonlehu.unwrap(),
      _ => return false,
    };

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
      let found = match self.klesi {
        Brarafsi => valsi.cmene.starts_with(&self.rafsi[0..4]),
        Gismu(_) => valsi.cmene == self.rafsi,
        _ => valsi.rafsi.contains(&self.rafsi),
      };

      if found {
        return Some(valsi);
      }
    }

    None
  }

  pub fn xu_sampu(&self) -> bool {
    use Raflei::*;

    match self.klesi {
      Gismu(_) | Brarafsi => false,
      _ => true,
    }
  }
}
