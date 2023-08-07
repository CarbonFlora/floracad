use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;

use crate::horizontal::HorizontalData;

impl HorizontalData {
    pub fn export_txt(&self) -> Result<()> {
        let mut file = File::create(self.input_directory.clone() + ".txt")?;
        let mut buf = String::new();
        let curve = self.to_horizontal_curve()?;

        buf += format!("\n{}", curve).as_str();

        write!(file, "{}", buf)?;
        Ok(())
    }

    pub fn export_pdf(&self) -> Result<()> {
        Ok(())
    }

    pub fn export_xlsx(&self) -> Result<()> {
        Err(anyhow!("temp failure."))
    }
}
