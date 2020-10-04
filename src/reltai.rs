use std::io::{stdout, BufWriter, Write};

use anyhow::Result;
use serde::Serialize;
use serde_json;

use crate::vanbi::{PrinaTarmi, Vanbi};

pub trait Reltai {
  fn ciska_tai_lahoi_json<W: Write>(&self, w: &mut W) -> Result<()>;
  fn ciska_tai_lo_vlapoi<W: Write>(&self, w: &mut W) -> Result<()>;

  fn prina(&self, vanbi: &Vanbi) -> Result<()> {
    let mut lo_ciska = BufWriter::new(stdout());

    match vanbi.prina_tarmi {
      PrinaTarmi::Json => self.ciska_tai_lahoi_json(&mut lo_ciska)?,
      PrinaTarmi::Text => self.ciska_tai_lo_vlapoi(&mut lo_ciska)?,
    }

    Ok(())
  }
}

impl<T: ToString + Serialize> Reltai for T {
  fn ciska_tai_lahoi_json<W: Write>(&self, w: &mut W) -> Result<()> {
    Ok(serde_json::to_writer(w, self)?)
  }

  fn ciska_tai_lo_vlapoi<W: Write>(&self, w: &mut W) -> Result<()> {
    Ok(write!(w, "{}", self.to_string())?)
  }
}
