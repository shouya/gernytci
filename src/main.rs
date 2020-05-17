use std::path::PathBuf;

use structopt::StructOpt;

// lei tutci
mod bixygau; // bixygau fi lo vlacku sfaile
mod tamsmi; // zvafa'i loi simsa valsi lo ka tarmi
mod zvafahi; // zvafa'i da poi mapti lo se sisku

mod kampu;
mod sidju;
mod vlacku;

use kampu::*;

#[derive(StructOpt)]
pub struct Tergalfi {
  #[structopt(
    name = "format",
    short,
    long,
    default_value = "text",
    help = "Output colored \"text\" or machine-readble \"json\""
  )]
  termontai: Termonai,

  #[structopt(
    name = "dict",
    short,
    long,
    parse(from_os_str),
    help = "Location to dict file",
    default_value = "[built-in]"
  )]
  vlacku: PathBuf,

  #[structopt(
    name = "offcial-only",
    help = "Use entries from @official_data only",
    short,
    long
  )]
  catni_poho: bool,

  #[structopt(subcommand)]
  minde: Minde,
}

#[derive(Clone, Copy)]
pub enum Termonai {
  Text,
  Json,
}

impl std::str::FromStr for Termonai {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self> {
    match s {
      "json" => Ok(Self::Json),
      "text" => Ok(Self::Text),
      _ => Err(anyhow!("only 'json' or 'text' is allowed")),
    }
  }
}

trait TciTeryruhe {
  fn termontai_lo_vlamei(&self);
  fn termontai_lahe_jeison(&self);

  fn ciska(&self, termontai: Termonai) {
    match termontai {
      Termonai::Text => self.termontai_lo_vlamei(),
      Termonai::Json => self.termontai_lahe_jeison(),
    }
  }
}

#[derive(StructOpt)]
#[structopt()]
pub enum Minde {
  Tamsmi(tamsmi::Tergalfi),
  Bixygau(bixygau::Tergalfi),
  Zvafahi(zvafahi::Tergalfi),
}

fn main() -> Result<()> {
  let tergalfi = Tergalfi::from_args();
  let mut vlacku = vlacku::Vlacku::tolsorcu(&tergalfi.vlacku)?;

  if tergalfi.catni_poho {
    vlacku.catni_poho();
  }

  match &tergalfi.minde {
    Minde::Tamsmi(_) => {
      tamsmi::tamsmi(&tergalfi, &vlacku)?.ciska(tergalfi.termontai)
    }
    Minde::Bixygau(_) => {
      bixygau::bixygau(&tergalfi, &mut vlacku)?.ciska(tergalfi.termontai)
    }
    Minde::Zvafahi(_) => {
      zvafahi::zvafahi(&tergalfi, &vlacku)?.ciska(tergalfi.termontai)
    }
  }

  Ok(())
}
