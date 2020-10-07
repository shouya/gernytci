use crate::kampu::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Brivla {
  Gismu(Gismu),
  Lujvo(Lujvo),
}

impl TryFrom<&str> for Brivla {
  type Error = anyhow::Error;

  fn try_from(lerpoi: &str) -> Result<Brivla> {
    let gismu = Gismu::try_from(lerpoi).map(|gismu| Brivla::Gismu(gismu));
    let lujvo = Lujvo::try_from(lerpoi).map(|lujvo| Brivla::Lujvo(lujvo));

    Ok(gismu.or(lujvo)?)
  }
}

impl ToString for Brivla {
  fn to_string(&self) -> String {
    use Brivla::*;

    match self {
      Gismu(gismu) => gismu.to_string(),
      Lujvo(lujvo) => lujvo.to_string()
    }
  }
}
