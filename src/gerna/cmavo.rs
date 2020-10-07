use crate::kampu::*;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Cmavo(String);

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct CmavoGirzu(Vec<Cmavo>);

impl TryFrom<&str> for Cmavo {
  type Error = anyhow::Error;

  fn try_from(lerpoi: &str) -> Result<Cmavo> {
    ensure!(!lerpoi.is_empty(), "a cmavo cannot be empty string");

    let mut lei_slaka = lerpoi.split('\'').collect_vec();
    let stedu = lei_slaka.remove(0);

    match Lerpoi::from(stedu).sance().as_str() {
      ".V" | "V" | "CV" | "CV'V" => (),
      "CVV" if Lerfu::relkarsna_sarxe(dbg!(&stedu[1..])) => (),
      "Cy" if lei_slaka.is_empty() => (),
      _ => bail!("cmavo cannot start with {}", stedu),
    }

    for slaka in lei_slaka.into_iter() {
      match Lerpoi::from(slaka).sance().as_str() {
        "V" => (),
        "VV" if Lerfu::relkarsna_sarxe(slaka) => (),
        "VV" => bail!("{} is not a valid diphthong", slaka),
        _ => bail!("invalid cmavo {}", lerpoi),
      }
    }

    Ok(Cmavo(lerpoi.to_string()))
  }
}


impl TryFrom<&str> for CmavoGirzu {
  type Error = anyhow::Error;

  fn try_from(lerpoi: &str) -> Result<CmavoGirzu> {
    ensure!(!lerpoi.is_empty(), "cmavo compound cannot be empty");

    ensure!(
      !Cmavo::vasru_zahumei_zunsna(lerpoi),
      "cmavo compound cannot contain consonant pair or cluster"
    );

    ensure!(
      Lerfu::from(lerpoi.chars().last().unwrap_or(' ')).sance() == 'V',
      "cmavo compound must end with a vowel"
    );

    let mut zasni = String::new();
    let mut cmavo_porsi = Vec::new();
    for lerfu in lerpoi.chars() {
      match Lerpoi::from(lerfu).sance().as_str() {
        "." | "V" | "'" => zasni.push(lerfu),
        "C" => {
          if !zasni.is_empty() {
            cmavo_porsi.push(Cmavo::try_from(zasni.as_str())?);
          }
          zasni.clear();
          zasni.push(lerfu);
        }
        lerfu => bail!("invalid lerfu found in cmavo {}", lerfu),
      }
    }

    cmavo_porsi.push(Cmavo::try_from(zasni.as_str())?);

    Ok(CmavoGirzu(cmavo_porsi))
  }
}

impl ToString for Cmavo {
  fn to_string(&self) -> String {
    self.0.clone()
  }
}

impl Cmavo {
  pub fn vasru_zahumei_zunsna(lerpoi: &str) -> bool {
    Lerpoi::from(lerpoi).sance().contains("CC")
  }
}
