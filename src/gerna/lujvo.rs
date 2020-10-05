use itertools::Itertools;

use crate::kampu::*;

#[derive(Clone, Serialize, Debug)]
pub struct Lujvo(Vec<Rafsi>);

impl Lujvo {
  pub fn iter(&self) -> impl Iterator<Item = &Rafsi> {
    self.0.iter()
  }

  fn pagbu_jvasahe(porsi: &[Rafsi]) -> bool {
    use Raflei::*;

    if porsi.len() <= 1 {
      return true;
    }

    // cipcta lo du'u loi rafsi remei ku jo'u lo terjonle'u cu sarxe
    for (seltau, tertau) in porsi.iter().tuple_windows() {
      let terjonlehu = seltau.terjonlehu;
      let pa = seltau.rafsi.chars().last().unwrap();
      let re = tertau.rafsi.chars().last().unwrap();

      if !(seltau.klesi().xu_sampu() && tertau.klesi().xu_sampu()) {
        continue;
      }

      if terjonlehu == Some('y') {
        if seltau.klesi() != CVC || Lerfu::lidne_zunsna_sarxe(pa, re) {
          return false;
        }
      }

      if terjonlehu == Some('n') || terjonlehu == Some('r') {
        if terjonlehu == Some('n') && re != 'r' {
          return false;
        }

        if !seltau.klesi().xu_tamsmi_cyvyvy() || tertau.klesi() == CCV {
          return false;
        }
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
