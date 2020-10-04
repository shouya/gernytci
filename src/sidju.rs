use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use anyhow::Result;
use flate2::read::GzDecoder;

pub fn tolsorcu_sfaile(pluta: &Path) -> Result<String> {
  let mut xadni = String::new();
  let mut sfaile = File::open(pluta)?;

  match Path::extension(pluta).and_then(OsStr::to_str) {
    Some("gz") => {
      GzDecoder::new(sfaile)?.read_to_string(&mut xadni)?;
    }
    _ => {
      sfaile.read_to_string(&mut xadni)?;
    }
  }

  Ok(xadni)
}

pub fn sorcu_sfaile(pluta: &Path, xadni: &str) -> Result<()> {
  File::create(pluta)?.write_all(xadni.as_bytes())?;
  Ok(())
}

pub fn jinzi_vlacku_sfaile() -> Result<String> {
  let jinzi: &'static [u8] = include_bytes!("../assets/vlacku.dict.gz");
  let mut xadni = String::new();
  GzDecoder::new(jinzi)?.read_to_string(&mut xadni)?;
  Ok(xadni)
}
