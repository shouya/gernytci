use crate::kampu::*;

use std::convert::From;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[allow(dead_code)]
pub enum Gimlei {
  CVCCV,
  CCVCV,
}

impl TryFrom<&str> for Gimlei {
  type Error = anyhow::Error;

  fn try_from(valsi: &str) -> Result<Self> {
    use Gimlei::*;
    match Lerpoi::from(valsi).sance().as_str() {
      "CVCCV" => Ok(CVCCV),
      "CCVCV" => Ok(CCVCV),
      _ => bail!("Invalid rafsi"),
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Gismu(String);

impl TryFrom<&str> for Gismu {
  type Error = anyhow::Error;

  fn try_from(valsi: &str) -> Result<Self> {
    match Lerpoi::from(valsi).sance().as_str() {
      "CVCCV" => Ok(Gismu(valsi.into())),
      "CCVCV" => Ok(Gismu(valsi.into())),
      _ => bail!("invalid gismu {}", valsi),
    }
  }
}

impl ToString for Gismu {
  fn to_string(&self) -> String {
    self.0.clone()
  }
}

impl Gismu {
  pub fn jvasahe(valsi: &str) -> bool {
    if valsi.len() != 5 {
      return false;
    }

    let gimlei = match Gimlei::try_from(valsi) {
      Ok(kelsi) => kelsi,
      _ => return false,
    };

    Self::lerpoi_jvasahe(valsi, gimlei)
  }

  pub fn lerpoi_jvasahe(valsi: &str, gimlei: Gimlei) -> bool {
    use Gimlei::*;

    match gimlei {
      CVCCV if Lerfu::zunsna_sarxe(&valsi[2..=3]) => true,
      CCVCV if Lerfu::zunsna_sarxe(&valsi[0..=1]) => true,
      _ => false,
    }
  }
}
