use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::kampu::Result;

pub fn tolsorcu_sfaile(pluta: &Path) -> Result<String> {
  let mut jalge = String::new();
  let _ = File::open(pluta)?.read_to_string(&mut jalge)?;
  Ok(jalge)
}

pub fn sorcu_sfaile(pluta: &Path, xadni: &str) -> Result<()> {
  File::create(pluta)?.write_all(xadni.as_bytes())?;
  Ok(())
}
