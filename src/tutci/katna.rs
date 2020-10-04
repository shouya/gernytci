use crate::kampu::*;

use itertools::Itertools;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Rafpoi(Vec<Rafsi>);

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  rafsi: Rafpoi,
  tanru: Vec<Option<Valsi>>,
}

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    let mut lerpoi = String::new();
    use colored::*;

    let rafsi: String = self
      .rafsi
      .cpacu()
      .iter()
      .map(|da| {
        format!(
          "{}{}",
          da.rafsi.blue(),
          da.terjonlehu
            .map(|x| x.to_string().white().to_string())
            .unwrap_or("".into())
        )
      })
      .collect::<Vec<_>>()
      .join("/");

    let tanru: String = self
      .tanru
      .iter()
      .zip(self.rafsi.cpacu().iter())
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

    lerpoi += &format!("{}\n", rafsi);
    lerpoi += &format!("{}\n", tanru);
    lerpoi += &format!("{}\n", glosa);
    lerpoi
  }
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<Teryruhe> {
  let lujvo = value_t!(selruhe, "lujvo", String).unwrap();
  let vlacku = vanbi.vlacku()?;
  match Rafpoi::genturfahi(&lujvo).as_slice() {
    [] => bail!("no valid tanru found"),
    [rafpoi] => Ok(Teryruhe {
      rafsi: rafpoi.clone(),
      tanru: rafpoi.vlaste_sisku(&vlacku),
    }),
    [..] => bail!("multiple results found"),
  }
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
  fn selpormei(&self) -> usize {
    use Raflei::*;
    match self {
      CVC | CCV | CVV => 3,
      CVhV => 4,
      Brarafsi => 4,
      Gismu(_) => 5,
    }
  }
}

impl Raflei {
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

  fn xu_tamsmi_cyvyvy(&self) -> bool {
    use Raflei::*;
    *self == CVV || *self == CVhV
  }

  fn mapti(rafsi: &str) -> Option<Raflei> {
    use Gimlei::*;
    use Raflei::*;

    if rafsi.len() < 3 || rafsi.len() > 5 {
      return None;
    }

    match lerfu_sanse(rafsi).as_str() {
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

impl Rafsi {
  fn selpormei(&self) -> usize {
    self.klesi.selpormei() + self.terjonlehu.map(|_| 1).unwrap_or(0)
  }

  // spuda fi loi romei cumki
  fn genturfahi_bavlahi(lujvo: &str) -> Vec<Self> {
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

  fn jvasahe(&self) -> bool {
    use Raflei::*;

    match self.klesi {
      Gismu(_) => self.terjonlehu == None,
      Brarafsi => self.terjonlehu == Some('y'),
      CVC | CCV | CVhV | CVV if self.terjonlehu == None => true,
      CVC | CCV | CVhV | CVV => self.rafsi_terjonlehu_jvasahe(),
    }
  }

  fn rafsi_terjonlehu_jvasahe(&self) -> bool {
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

  fn vlaste_sisku(&self, vlacku: &Vlacku) -> Option<Valsi> {
    use Raflei::*;

    for valsi in vlacku.iter() {
      let found = match self.klesi {
        Brarafsi => valsi.cmene.starts_with(&self.rafsi[0..4]),
        Gismu(_) => valsi.cmene == self.rafsi,
        _ => valsi.rafsi.contains(&self.rafsi),
      };

      if found {
        return Some(valsi.clone());
      }
    }

    None
  }

  fn xu_sampu(&self) -> bool {
    use Raflei::*;

    match self.klesi {
      Gismu(_) | Brarafsi => false,
      _ => true,
    }
  }
}

impl Rafpoi {
  fn kunti() -> Self {
    Self(vec![])
  }

  fn cpacu(&self) -> &Vec<Rafsi> {
    &self.0
  }

  fn stedu_setca(&mut self, rafsi: Rafsi) {
    self.0 = [vec![rafsi], self.0.clone()].concat();
  }

  fn pagbu_jvasahe(&self) -> bool {
    use Raflei::*;

    let porsi = &self.0;

    if porsi.len() <= 1 {
      return true;
    }

    // cipcta lo du'u loi rafsi remei ku jo'u lo terjonle'u cu sarxe
    for (seltau, tertau) in porsi.iter().tuple_windows() {
      let terjonlehu = seltau.terjonlehu;
      let pa = seltau.rafsi.chars().last().unwrap();
      let re = tertau.rafsi.chars().last().unwrap();

      if !(seltau.xu_sampu() && tertau.xu_sampu()) {
        continue;
      }

      if terjonlehu == Some('y') {
        if seltau.klesi != CVC || CURMI_ZUNSNA_REMEI.contains(&(pa, re)) {
          return false;
        }
      }

      if terjonlehu == Some('n') || terjonlehu == Some('r') {
        if terjonlehu == Some('n') && re != 'r' {
          return false;
        }

        if !seltau.klesi.xu_tamsmi_cyvyvy() || tertau.klesi == CCV {
          return false;
        }
      }
    }

    true
  }

  fn jvasahe(&self) -> bool {
    use Raflei::*;

    if !self.pagbu_jvasahe() {
      return false;
    }

    if self.0.len() < 2 {
      return false;
    }

    // no sumti cu naku ka'e zvati da'a lo mulfa'o
    let (romoi, loi_drata) = self.0.as_slice().split_last().unwrap();
    for rafsi in loi_drata {
      if let Gismu(_) = rafsi.klesi {
        return false;
      }
    }

    if let Some(_) = romoi.terjonlehu {
      return false;
    }

    true
  }

  fn genturfahi(lujvo: &str) -> Vec<Self> {
    Self::pagbu_genturfahi(lujvo)
      .into_iter()
      .filter(|x| x.jvasahe())
      .collect()
  }

  fn pagbu_genturfahi(lujvo: &str) -> Vec<Self> {
    if lujvo.len() == 0 {
      return vec![Self::kunti()];
    }

    let mut teryruhe = vec![];

    for rafsi in Rafsi::genturfahi_bavlahi(lujvo) {
      let velvihu = &lujvo[rafsi.selpormei()..];
      for mut lerpoi in Self::pagbu_genturfahi(velvihu) {
        lerpoi.stedu_setca(rafsi.clone());
        teryruhe.push(lerpoi);
      }
    }

    teryruhe.into_iter().filter(|x| x.pagbu_jvasahe()).collect()
  }

  fn vlaste_sisku(&self, vlaste: &Vlacku) -> Vec<Option<Valsi>> {
    self.0.iter().map(|x| x.vlaste_sisku(vlaste)).collect()
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
  terjonlehu: Option<char>,
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

fn lerfu_sanse(valsi: &str) -> String {
  let mut teryruhe = String::new();
  for lerfu in valsi.to_lowercase().chars() {
    if let Some(_) = "aeiou".find(lerfu) {
      teryruhe.push('V');
      continue;
    }
    if let Some(_) = "bcdfgjklmnprstvxz".find(lerfu) {
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
    xusra_katna("saiclire", "sai/clire");
    xusra_katna("saicli", "sai/cli");
    xusra_katna("sanmycli", "sanmy/cli");
    xusra_katna("sanmycilre", "sanmy/cilre");
    xusra_katna("zvaju'o", "zva/ju'o");
    xusra_katna("ju'ozva", "ju'o/zva");
    xusra_katna("cmeterge'a", "cme/ter/ge'a");

    xusra_katna("famyma'o", "famy/ma'o");
    xusra_katna("mitysisku", "mity/sisku");

    xusra_katna("ba'urdjica", "ba'ur/djica");
    xusra_katna("ba'urdu'u", "ba'ur/du'u");

    xusra_naljvasahe("saiycli");
    xusra_naljvasahe("saircli");
    xusra_naljvasahe("saincli");

    xusra_naljvasahe("barda");
    xusra_naljvasahe("dit");
    xusra_naljvasahe("dity");
    xusra_naljvasahe("skamiskami");
  }

  fn xusra_naljvasahe(lujvo: &str) {
    assert!(Rafpoi::genturfahi(lujvo).as_slice().len() == 0)
  }
  fn xusra_katna(lujvo: &str, lei_rafsi: &str) {
    if let [rafpoi] = Rafpoi::genturfahi(lujvo).as_slice() {
      assert_eq!(
        rafpoi.cpacu().iter().map(|x| x.to_string()).join("/"),
        lei_rafsi
      );
    } else {
      println!(
        "{} => {}, got: {:?}",
        lujvo,
        lei_rafsi,
        Rafpoi::genturfahi(lujvo)
      );
      assert!(false)
    }
  }
}
