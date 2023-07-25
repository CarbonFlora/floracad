use std::fmt;
use crate::datatypes::Station;

pub mod calculate;
pub mod interval;

use calculate::VerticalCurve;

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

impl fmt::Display for VerticalCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Curve Length: {}", self.dimensions.curve_length)?;
        writeln!(f, "Grade: {} -> {}", self.dimensions.incoming_grade, self.dimensions.outgoing_grade)?;
        writeln!(f, "External: {}", self.dimensions.external)?;
        // writeln!(f, "Curve Length: {}", self.dimensions.sight_distance)?; todo!()
        writeln!(f, "\n~ Major Stations\n{}{}{}", self.stations.pvc, self.stations.pvi, self.stations.pvt)?;
        Ok(())
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "STA: {}, ELEV: {}", self.value, self.elevation)?;
        Ok(())
    }
}