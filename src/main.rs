use std::path::PathBuf;

use structopt::StructOpt;

// lei tutci
mod bixygau;
mod tamsmi; // zvafa'i loi simsa valsi lo ka tarmi

mod kampu;
mod sidju;
mod vlacku;

use kampu::*;

#[derive(StructOpt)]
pub struct Tergalfi {
  #[structopt(name = "format", short, long, default_value = "text")]
  termontai: Termonai,

  #[structopt(
    name = "dict",
    short,
    long,
    parse(from_os_str),
    default_value = "./vlacku.dict"
  )]
  vlacku: PathBuf,

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
}

fn main() -> Result<()> {
  let tergalfi = Tergalfi::from_args();
  let mut vlacku = vlacku::Vlacku::tolsorcu(&tergalfi.vlacku)?;

  match &tergalfi.minde {
    Minde::Tamsmi(_) => {
      tamsmi::tamsmi(&tergalfi, &vlacku)?.ciska(tergalfi.termontai)
    }
    Minde::Bixygau(_) => {
      bixygau::bixygau(&tergalfi, &mut vlacku)?.ciska(tergalfi.termontai)
    }
  }

  Ok(())
}
