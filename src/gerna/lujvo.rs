use itertools::Itertools;

use crate::gerna::lerfu::CURMI_ZUNSNA_REMEI;
use crate::kampu::*;

#[derive(Clone, Serialize, Debug)]
pub struct Lujvo(Vec<Rafsi>);

impl Lujvo {
  fn kunti() -> Self {
    Self(vec![])
  }

  pub fn iter(&self) -> impl Iterator<Item = &Rafsi> {
    self.0.iter()
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

  pub fn genturfahi(lujvo: &str) -> Vec<Self> {
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

  pub fn vlaste_sisku(&self, vlaste: &Vlacku) -> Vec<Option<Valsi>> {
    self
      .0
      .iter()
      .map(|x| x.vlaste_sisku(vlaste).map(|x| x.clone()))
      .collect()
  }
}
