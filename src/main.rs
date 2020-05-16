use structopt::StructOpt;
use std::path::PathBuf;

// lei tutci
mod tamsmi; // zvafa'i loi simsa valsi lo ka tarmi

mod sidju;
mod kampu;
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
  minde: Minde
}

pub enum Termonai {
  Text,
  Json
}

impl std::str::FromStr for Termonai {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self> {
    match s {
      "json" => Ok(Self::Json),
      "text" => Ok(Self::Text),
      _ => Err(anyhow!("only 'json' or 'text' is allowed"))
    }
  }
}

#[derive(StructOpt)]
#[structopt()]
pub enum Minde {
  Tamsmi(tamsmi::Tergalfi)
}


fn main() -> Result<()> {
  let _tergalfi = Tergalfi::from_args();

  unimplemented!()
}
