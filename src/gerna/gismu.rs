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

  fn try_from(s: &str) -> Result<Self> {
    use Gimlei::*;
    match Lerpoi::from(s).sance().as_str() {
      "CVCCV" => Ok(CVCCV),
      "CCVCV" => Ok(CCVCV),
      _ => bail!("Invalid rafsi"),
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Gismu(String);

impl<T> From<T> for Gismu
where
  T: Into<String>,
{
  fn from(krati: T) -> Self {
    Self(krati.into())
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