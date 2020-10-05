use derive_more::{From, Into};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Lerpoi(String);

#[derive(Debug, Clone, From, Into, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Lerfu(char);

impl<T> From<T> for Lerpoi
where
  T: Into<String>,
{
  fn from(da: T) -> Self {
    Lerpoi(da.into())
  }
}

impl Lerfu {
  pub fn sance(&self) -> char {
    let se_gaheltoha = self.0.to_ascii_lowercase();
    if let Some(_) = "aeiou".find(se_gaheltoha) {
      'V'
    } else if let Some(_) = "bcdfgjklmnprstvxz".find(se_gaheltoha) {
      'C'
    } else {
      self.0
    }
  }

  pub fn zunsna_sarxe(remei: &str) -> bool {
    let pamoi = remei.chars().nth(0).unwrap();
    let remoi = remei.chars().nth(1).unwrap();

    // zoi gy. It is forbidden for both consonants to be the same
    // gy. ra'i la .cylyly.
    if pamoi == remoi {
      return false;
    }

    // zoi gy. It is forbidden for one consonant to be voiced and the
    // other unvoiced .gy ra'i la .cylyly.
    if VOKSA_ZUNSNA.contains(pamoi) && VOKCAU_ZUNSNA.contains(remoi)
      || VOKSA_ZUNSNA.contains(remoi) && VOKCAU_ZUNSNA.contains(pamoi)
    {
      return false;
    }

    // zoi gy. It is forbidden for both consonants to be drawn from
    // the set c, j, s, z. .gy ra'i la .cylyly.
    if "cjsz".contains(pamoi) && "cjsz".contains(remoi) {
      return false;
    }

    // zoi gy. The specific pairs cx, kx, xc, xk, and mz are forbidden .gy
    if ["cx", "kx", "xc", "xk", "mz"].contains(&remei) {
      return false;
    }

    return true;
  }

  pub fn lidne_zunsna_sarxe(remei: &str) -> bool {
    return CURMI_ZUNSNA_REMEI.contains(&&remei[..]);
  }

  pub fn zunsna_cimei_sarxe(cimei: &str) -> bool {
    // zoi gy. The first two consonants must constitute a permissible
    // consonant pair .gy
    if !Self::zunsna_sarxe(&cimei[0..=1]) {
      return false;
    }

    // zoi gy. The last two consonants must constitute a permissible
    // initial consonant pair .gy
    if !Self::lidne_zunsna_sarxe(&cimei[1..=2]) {
      return false;
    }

    if ["ndj", "ndz", "ntc", "nts"].contains(&&cimei[..]) {
      return false;
    }

    true
  }
}

impl Lerpoi {
  pub fn sance(&self) -> String {
    self.0.chars().map(|c| Lerfu::from(c).sance()).collect()
  }
}

// zoi gy. voiced consonants .gy
const VOKSA_ZUNSNA: &'static str = "bdgvjz";

// zoi gy. unvoiced consonants .gy
const VOKCAU_ZUNSNA: &'static str = "ptkfcsx";

// zoi gy. permissible initial consonant pairs .gy
const CURMI_ZUNSNA_REMEI: &[&'static str] = &[
  "pl", "pr", "bl", "br", "vl", "vr", "cp", "cf", "ct", "ck", "cm", "cn", "cl",
  "cr", "jb", "jv", "jd", "jg", "jm", "sp", "sf", "st", "sk", "sm", "sn", "sl",
  "sr", "zb", "zv", "zd", "zg", "zm", "tc", "tr", "ts", "kl", "kr", "dj", "dr",
  "dz", "gl", "gr", "ml", "mr", "xl", "xr",
];
