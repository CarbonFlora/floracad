use anyhow::{anyhow, Result};
use chrono::Local;
use std::fs::File;
use std::io::prelude::*;

use crate::datatypes::*;
use crate::horizontal::HorizontalData;

impl HorizontalData {
    pub fn export_txt(&self) -> Result<()> {
        let mut file = File::create(self.input_directory.clone() + ".txt")?;
        let mut buf = String::new();
        let curve = self.to_horizontal_curve()?;
        buf += format!("\nCurve Details\n--\n{}", curve.dimensions).as_str();
        buf += format!("\nMajor Stations\n--\n{}", curve.stations).as_str();

        if !self.input_design_speed.is_empty() && !self.input_m.is_empty() {
            match curve.is_compliant(
                self.input_design_standard,
                self.input_sight_type,
                calc_adjustment(self.sustained_downgrade),
            ) {
                Err(e) => {}
                Ok(j) => {
                    if j.0 {
                        buf += format!(
                            "\nSight Distance Validation ({:?} - {:?})\n--\n[COMPLIANT]{:.2} > {:.2}",
                            self.input_design_standard,
                            self.input_sight_type,
                            curve.dimensions.sight_distance,
                            j.1
                        )
                        .as_str();
                    } else {
                        buf += format!(
                            "\nSight Distance Validation ({:?} - {:?})\n--\n[NONCOMPLIANT]{:.2} < {:.2}",
                            self.input_design_standard,
                            self.input_sight_type,
                            curve.dimensions.sight_distance,
                            j.1
                        )
                        .as_str();
                    }
                }
            }
        }

        if !self.input_station_interval.is_empty()
            && coerce_station_value(&self.input_station_interval).is_ok()
        {
            let t = coerce_station_value(&self.input_station_interval).unwrap_or_default();
            buf += format!("\n\nInterval Stations\n--\n{}", curve.interval_stations(t)).as_str();
        }

        buf += format!("\nDate of Production: {}", Local::now()).as_str();
        buf += format!("\n\nSource Data\n--\n{:?}", self).as_str();

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
