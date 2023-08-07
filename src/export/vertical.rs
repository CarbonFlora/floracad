use anyhow::{anyhow, Result};

use crate::vertical::VerticalData;

impl VerticalData {
    pub fn export_txt(&self) -> Result<()> {
        Ok(())
    }

    pub fn export_pdf(&self) -> Result<()> {
        Ok(())
    }

    pub fn export_xlsx(&self) -> Result<()> {
        Err(anyhow!("temp failure."))
    }
}
