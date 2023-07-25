use std::fmt;
use crate::datatypes::Station;

pub mod calculate;
pub mod interval;

use calculate::VerticalCurve;

use self::{interval::CurveDetail, calculate::{VerticalDimensions, VerticalStations}};

#[derive(Debug, Clone, Copy)]
pub enum VerticalDefinition {
    PVI,
    PVC,
    PVT,
}

#[derive(Debug, Clone)]
pub struct VerticalData {
    pub input_method: VerticalDefinition,
    pub input_station: String,
    pub input_elevation: String,
    pub input_incoming_grade: String,
    pub input_outgoing_grade: String,
    pub input_length: String,
    pub input_station_interval: String,
}

impl VerticalDefinition {
    pub fn next(self) -> Self {
        match self {
            VerticalDefinition::PVC => VerticalDefinition::PVI,
            VerticalDefinition::PVI => VerticalDefinition::PVT,
            VerticalDefinition::PVT => VerticalDefinition::PVC,
        }
    } 
}

impl fmt::Display for VerticalDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "~ Curve Details")?;
        writeln!(f, "Curve Length: {:.2}", self.curve_length)?;
        writeln!(f, "Grade: {:.2}% -> {:.2}%", self.incoming_grade*100.0, self.outgoing_grade*100.0)?;
        writeln!(f, "External: {:.2}", self.external.abs())?;
        // writeln!(f, "Curve Length: {}", self.dimensions.sight_distance)?; todo!()
        Ok(())
    }
}

impl fmt::Display for VerticalStations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "~ Major Stations")?;
        write!(f, "PVC > {:.2}", self.pvc)?;
        write!(f, "PVI > {:.2}", self.pvi)?;
        write!(f, "PVT > {:.2}", self.pvt)?; 
        Ok(())
    }
}

impl fmt::Display for VerticalCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.dimensions)?;
        writeln!(f, "{}", self.stations)?;
        Ok(())
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "STA: {:.0}+{:.2}, ELEV: {:.2}", (self.value/100.0).trunc(), self.value-(self.value/100.0).trunc()*100.0, self.elevation)?;
        Ok(())
    }
}

impl fmt::Display for CurveDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "~ Interval Stations")?;
        for station in &self.interval {
            write!(f, "> {:.2}", station)?;
        }
        Ok(())
    }
}