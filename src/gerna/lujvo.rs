use itertools::Itertools;

use crate::kampu::*;

#[derive(Clone, Serialize, From, Debug)]
pub struct Lujvo(Vec<Rafsi>);

macro_rules! bapli {
  ($e:expr) => {
    if !($e) {
      return false;
    }
  };
}

impl Lujvo {
  pub fn iter(&self) -> impl Iterator<Item = &Rafsi> {
    self.0.iter()
  }

  #[allow(dead_code)]
  fn finti(porsi: &[&str]) -> Result<Lujvo> {
    let rafsi_liste: Vec<Rafsi> = porsi
      .iter()
      .map(|x| Rafsi::zbasu(x, None))
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
      CCV | CVC | CVV | CVhV => (),
      GismuRafsi(_) => (),
      Brarafsi(_) => bail!("last rafsi in a lujvo cannot be 4-letter"),
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

    for (seltau, tertau) in liste.iter_mut().zip(liste_fukpi.iter()) {
      let lerpoi = format!("{}{}", &seltau.rafsi[..=0], &tertau.rafsi[1..=1]);
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

    for (pamoi, remoi) in liste.iter().tuple_windows() {
      if pamoi.klesi() != CVC {
        return Ok(());
      }

      if pamoi.terjonlehu == Some('y') || remoi.klesi() == GismuRafsi(CVCCV) {
        break;
      }

      let lerpoi = format!("{}{}", &pamoi.rafsi[..=0], &remoi.rafsi[1..=1]);
      if !Lerfu::lidne_zunsna_sarxe(&lerpoi) {
        return Ok(());
      }
    }

    liste[0].terjonlehu = Some('y');
    Ok(())
  }

  fn pagbu_jvasahe(porsi: &[Rafsi]) -> bool {
    use Raflei::*;

    if porsi.len() <= 1 {
      return true;
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
          bapli!(porsi[0].terjonlehu.is_none())
        } else {
          bapli!(
            porsi[0].terjonlehu == Some('n')
              || porsi[0].terjonlehu == Some('r')
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
      let terjonlehu = seltau.terjonlehu;
      let remei = format!(
        "{}{}",
        &seltau.rafsi[seltau.rafsi.len() - 1..],
        &tertau.rafsi[..1]
      );

      if !(seltau.klesi().xu_sampu() && tertau.klesi().xu_sampu()) {
        continue;
      }

      if !Lerfu::zunsna_sarxe(&remei) {
        bapli!(!terjonlehu.is_none())
      }
    }

    true
  }

  fn jvasahe(porsi: &[Rafsi]) -> bool {
    use Raflei::GismuRafsi;

    if !Self::pagbu_jvasahe(porsi) {
      return false;
    }

    if porsi.len() < 2 {
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
      .into_iter()
      .filter(|x| Self::pagbu_jvasahe(x))
      .collect()
  }

  pub fn vlaste_sisku(&self, vlaste: &Vlacku) -> Vec<Option<Valsi>> {
    self
      .0
      .iter()
      .map(|x| x.vlaste_sisku(vlaste).map(|x| x.clone()))
      .collect()
  }
}
