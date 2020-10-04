use crate::kampu::*;

#[derive(Clone, Serialize, Debug)]
pub struct Teryruhe {
  lujvo: Lujvo,
  tanru: Vec<Option<Valsi>>,
}

impl ToString for Teryruhe {
  fn to_string(&self) -> String {
    let mut lerpoi = String::new();
    use colored::*;

    let lujvo: String = self
      .lujvo
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
      .zip(self.lujvo.iter())
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

    lerpoi += &format!("{}\n", lujvo);
    lerpoi += &format!("{}\n", tanru);
    lerpoi += &format!("{}\n", glosa);
    lerpoi
  }
}

pub fn pruce(selruhe: &ArgMatches, vanbi: &Vanbi) -> Result<Teryruhe> {
  let lujvo = value_t!(selruhe, "lujvo", String).unwrap();
  let vlacku = vanbi.vlacku()?;
  match Lujvo::genturfahi(&lujvo).as_slice() {
    [] => bail!("no valid tanru found"),
    [lujvo] => Ok(Teryruhe {
      lujvo: lujvo.clone(),
      tanru: lujvo.vlaste_sisku(&vlacku),
    }),
    [..] => bail!("multiple results found"),
  }
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

    xusra_katna("ri'ornimre", "ri'or/nimre");

    xusra_naljvasahe("saiycli");
    xusra_naljvasahe("saircli");
    xusra_naljvasahe("saincli");

    xusra_naljvasahe("barda");
    xusra_naljvasahe("dit");
    xusra_naljvasahe("dity");
    xusra_naljvasahe("skamiskami");
  }

  fn xusra_naljvasahe(lujvo: &str) {
    assert!(Tanru::genturfahi(lujvo).as_slice().len() == 0)
  }
  fn xusra_katna(lujvo: &str, lei_rafsi: &str) {
    if let [Tanru] = Tanru::genturfahi(lujvo).as_slice() {
      assert_eq!(
        Tanru.cpacu().iter().map(|x| x.to_string()).join("/"),
        lei_rafsi
      );
    } else {
      println!(
        "{} => {}, got: {:#?}",
        lujvo,
        lei_rafsi,
        Tanru::genturfahi(lujvo)
      );
      assert!(false)
    }
  }
}
