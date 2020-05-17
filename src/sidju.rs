use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use flate2::read::GzDecoder;

use crate::kampu::Result;

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
