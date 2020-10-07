use itertools::Itertools;
use std::convert::TryFrom;

use crate::kampu::*;

#[derive(Clone, Serialize, From, Debug, PartialEq)]
pub struct Lujvo(Vec<Rafsi>);

macro_rules! bapli {
  ($e:expr) => {
    if !($e) {
      return false;
    }
  };
}

impl ToString for Lujvo {
  fn to_string(&self) -> String {
    self.0.iter().map(Rafsi::to_string).collect()
  }
}

impl TryFrom<&str> for Lujvo {
  type Error = anyhow::Error;
  fn try_from(t: &str) -> Result<Self> {
    Self::genturfahi(t)
      .pop()
      .ok_or_else(|| anyhow!("invalid lujvo"))
  }
}

impl Lujvo {
  pub fn iter(&self) -> impl Iterator<Item = &Rafsi> {
    self.0.iter()
  }

  pub fn finti<T: AsRef<str>>(porsi: &[T]) -> Result<Lujvo> {
    let rafsi_liste: Vec<Rafsi> = porsi
      .iter()
      .map(|x| Rafsi::zbasu(x.as_ref(), None))
      .collect::<Result<_>>()?;

    let lujvo = Self::jongau(rafsi_liste)?;

    Ok(lujvo)
  }

  fn jongau(mut liste: Vec<Rafsi>) -> Result<Lujvo> {
    Self::cipra_rafsi_tarmi(&liste[..])?;
    Self::cyvyvy_rafsi_jongau(&mut liste[..])?;
    Self::fendi_zunsna_jongau(&mut liste[..])?;
    Self::fendi_brarafsi_jongau(&mut liste[..])?;
    Self::cikre_tosmabru(&mut liste[..])?;

    Ok(liste.into())
  }

  fn cipra_rafsi_tarmi(liste: &[Rafsi]) -> Result<()> {
    use Raflei::*;
    ensure!(!liste.is_empty(), "list of rafsi cannot be empty");

    let romoi = liste.last().unwrap();

    match romoi.klesi() {
      CCV | CVV | CVhV => (),
      GismuRafsi(_) => (),
      Brarafsi(_) => bail!("last rafsi in a lujvo cannot be 4-letter"),
      CVC => bail!("last rafsi cannot be of CVC form"),
    }

    for rafsi in liste.iter().take(liste.len() - 1) {
      match rafsi.klesi() {
        GismuRafsi(_) => bail!("gismu cannot appear in the middle of a lujvo"),
        _ => (),
      }
    }

    Ok(())
  }

  fn cyvyvy_rafsi_jongau(liste: &mut [Rafsi]) -> Result<()> {
    use Raflei::*;

    let mut nitcu_pamoi_terjonlehu = false;
    if liste.len() > 3 {
      // zoi gy. If there are more than two words in the tanru, put an
      // r-hyphen (or an n-hyphen) after the first rafsi if it is
      // CVV-form .gy
      if liste[0].klesi().xu_tamsmi_cyvyvy() {
        nitcu_pamoi_terjonlehu = true;
      }
    } else if liste.len() == 2 {
      // zoi gy. If there are exactly two words, then put an r-hyphen
      // (or an n-hyphen) between the two rafsi if the first rafsi is
      // CVV-form, unless the second rafsi is CCV-form (for example,
      // saicli requires no hyphen) .gy
      if liste[0].klesi().xu_tamsmi_cyvyvy() && liste[1].klesi() != CCV {
        nitcu_pamoi_terjonlehu = true;
      }
    }

    // zoi gy. Use an r-hyphen unless the letter after the hyphen is
    // r, in which case use an n-hyphen. Never use an n-hyphen unless
    // it is required .gy
    if nitcu_pamoi_terjonlehu {
      if liste[1].rafsi.chars().next() == Some('r') {
        liste[0].terjonlehu = Some('n');
      } else {
        liste[0].terjonlehu = Some('r');
      }
    }

    Ok(())
  }

  fn fendi_zunsna_jongau(liste: &mut [Rafsi]) -> Result<()> {
    let liste_fukpi = liste.to_vec();

    for (seltau, tertau) in liste.iter_mut().zip(liste_fukpi.iter().skip(1)) {
      let lerpoi = format!("{}{}", seltau.romoi_lerfu(), tertau.pamoi_lerfu());
      // zoi gy. Put a y-hyphen between the consonants of any
      // impermissible consonant pair. This will always appear between
      // rafsi .gy
      if !Lerfu::zunsna_sarxe(&lerpoi) {
        seltau.terjonlehu = Some('y')
      }
    }

    Ok(())
  }

  fn fendi_brarafsi_jongau(liste: &mut [Rafsi]) -> Result<()> {
    use Raflei::*;
    for rafsi in liste.iter_mut() {
      if let Some(_) = rafsi.terjonlehu {
        continue;
      }

      // zoi gy. Put a y-hyphen after any 4-letter rafsi form .gy
      match rafsi.klesi() {
        Brarafsi(_) => rafsi.terjonlehu = Some('y'),
        _ => continue,
      }
    }

    Ok(())
  }

  fn cikre_tosmabru(liste: &mut [Rafsi]) -> Result<()> {
    use Gimlei::*;
    use Raflei::*;

    let mut tosmabru = false;
    for (pamoi, remoi) in liste.iter().tuple_windows() {
      if pamoi.klesi() != CVC {
        return Ok(());
      }

      let lerpoi = format!("{}{}", pamoi.romoi_lerfu(), remoi.pamoi_lerfu());
      if !Lerfu::lidne_zunsna_sarxe(&lerpoi) {
        return Ok(());
      }

      if pamoi.terjonlehu == Some('y') || remoi.klesi() == GismuRafsi(CVCCV) {
        tosmabru = true;
        break;
      }
    }

    if tosmabru {
      liste[0].terjonlehu = Some('y');
    }

    Ok(())
  }

  fn pagbu_jvasahe(porsi: &[Rafsi]) -> bool {
    use Raflei::*;

    if porsi.len() < 2 {
      return false;
    }

    // zoi gy. If there are more than two words in the tanru, put an
    // r-hyphen (or an n-hyphen) after the first rafsi if it is
    // CVV-form. .gy
    if porsi.len() > 2 {
      if porsi[0].klesi().xu_tamsmi_cyvyvy() {
        match porsi[0].terjonlehu {
          Some('n') | Some('r') => (),
          _ => return false,
        }
      }
    }

    // zoi gy. If there are exactly two words, then put an r-hyphen
    // (or an n-hyphen) between the two rafsi if the first rafsi is
    // CVV-form, unless the second rafsi is CCV-form (for example,
    // saicli requires no hyphen) .gy
    if porsi.len() == 2 {
      if porsi[0].klesi().xu_tamsmi_cyvyvy() {
        if porsi[1].klesi() == CCV {
          if porsi[0].terjonlehu.is_none() {
            return true;
          } else {
            return false;
          }
        } else {
          bapli!(
            porsi[0].terjonlehu == Some('n')
              || porsi[0].terjonlehu == Some('r')
              || porsi[0].terjonlehu == None
          )
        }

        // zoi gy. Use an r-hyphen unless the letter after the hyphen
        // is r, in which case use an n-hyphen. Never use an n-hyphen
        // unless it is required .gy
        if porsi[1].rafsi.chars().next().unwrap() != 'r' {
          bapli!(porsi[0].terjonlehu == Some('r'))
        } else {
          bapli!(porsi[0].terjonlehu == Some('n'))
        }
      }
    }

    // cipcta lo du'u loi rafsi remei ku jo'u lo terjonle'u cu sarxe
    for (seltau, tertau) in porsi.iter().tuple_windows() {
      let remei = format!(
        "{}{}",
        &seltau.rafsi[seltau.rafsi.len() - 1..],
        &tertau.rafsi[..1]
      );

      match seltau.klesi() {
        Brarafsi(_) => bapli!(seltau.terjonlehu == Some('y')),
        CVC => {
          if Lerfu::zunsna_sarxe(&remei) {
            match seltau.terjonlehu {
              // naku mulno drani .i cafne fa lo ka banzu be fa lo
              // du'u cipra fa tu'a la'o gy. None .gy va'o lodu'u lei
              // lerfu cu zunsna sarxe
              //
              // i ku'i tu'a lo zoi gy. tosmabru .gy fliba cu curmi lo
              // cumki je po'oje'u cumki ku be le terjonle'u po'u zo
              // ybu
              //
              // i mi lazni lo ka ciska lo javni poi traji lo ka drani
              // i le'i nu pruce ku cu banzu fi so'a cumki vau pe'i
              Some('y') | None => (),
              _ => return false,
            }
          }
          if !Lerfu::zunsna_sarxe(&remei) && seltau.terjonlehu != Some('y') {
            return false;
          }
        }
        _ => continue,
      };
    }

    true
  }

  fn jvasahe(porsi: &[Rafsi]) -> bool {
    use Raflei::GismuRafsi;

    if !Self::pagbu_jvasahe(porsi) {
      return false;
    }

    if porsi.len() < 1 {
      return false;
    }

    // no sumti cu naku ka'e zvati da'a lo mulfa'o
    let (romoi, loi_drata) = porsi.split_last().unwrap();
    for rafsi in loi_drata {
      if let GismuRafsi(_) = rafsi.klesi() {
        return false;
      }
    }

    if let Some(_) = romoi.terjonlehu {
      return false;
    }

    true
  }

  pub fn genturfahi(lujvo: &str) -> Vec<Self> {
    Self::pagbu_genturfahi(lujvo)
      .into_iter()
      .filter(|x| Self::jvasahe(&x))
      .map(|x| Self(x))
      .collect()
  }

  fn pagbu_genturfahi(lujvo: &str) -> Vec<Vec<Rafsi>> {
    if lujvo.len() == 0 {
      return vec![vec![]];
    }

    let mut teryruhe = vec![];

    for rafsi in Rafsi::genturfahi_bavlahi(lujvo) {
      let velvihu = &lujvo[rafsi.selpormei()..];
      for mut lerpoi in Self::pagbu_genturfahi(velvihu) {
        lerpoi.insert(0, rafsi.clone());
        teryruhe.push(lerpoi);
      }
    }

    teryruhe
  }

  pub fn vlaste_sisku(&self, vlaste: &Vlacku) -> Vec<Option<Valsi>> {
    self
      .0
      .iter()
      .map(|x| x.vlaste_sisku(vlaste).map(|x| x.clone()))
      .collect()
  }

  pub fn jvovahi(&self) -> usize {
    use Gimlei::*;
    use Raflei::*;

    // zoi gy. 1. Count the total number of letters, including hyphens
    // and apostrophes; call it L .gy
    let l = self.to_string().len();
    // zoi gy. 2. Count the number of apostrophes; call it A .gy
    let a = self.to_string().chars().filter(|x| *x == '\'').count();
    // zoi gy. 3. Count the number of y-, r-, and n-hyphens; call it H
    // .gy
    let h = self.0.iter().flat_map(|x| x.terjonlehu).count();

    // zoi gy. 4. For each rafsi, find the value in the following
    // table. Sum this value over all rafsi; call it R gy
    let r: usize = self
      .0
      .iter()
      .map(|x| match x.klesi() {
        // zo sarji mu'a
        GismuRafsi(CVCCV) => 1,
        // ra'oi sarj mu'a
        Brarafsi(CVCCV) => 2,
        // zo zbasu mu'a
        GismuRafsi(CCVCV) => 3,
        // ra'oi zbas mu'a
        Brarafsi(CCVCV) => 4,
        // ra'oi nun mu'a
        CVC => 5,
        // ra'oi ta'u mu'a
        CVhV => 6,
        // ra'oi zba mu'a
        CCV => 7,
        // ra'oi sai mu'a
        CVV => 8,
      })
      .sum();

    // zoi gy. 5 Count the number of vowels, not including y; call it
    // V .gy
    let v = self
      .to_string()
      .chars()
      .filter(|x| "aeiou".contains(*x))
      .collect::<String>()
      .len();

    // zoi gy. The score is then: (1000 * L) - (500 * A) + (100 * H) -
    // (10 * R) - V .gy
    1000 * l - 500 * a + 100 * h - 10 * r - v
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn jvovahi_cu_satci() {
    let jvovahi = |jvo: &str| Lujvo::try_from(jvo).unwrap().jvovahi();

    assert_eq!(jvovahi("zbasai"), 5847);
    assert_eq!(jvovahi("nunynau"), 6967);
    assert_eq!(jvovahi("sairzbata'u"), 10385);
    assert_eq!(jvovahi("zbazbasysarji"), 12976);
  }
}
