use anyhow::Result;

use crate::datatypes::*;

pub mod calculate;
pub mod interval;
pub mod display;

use self::calculate::*;
use self::interval::*;


#[derive(Debug, Clone, Copy)]
pub enum VerticalDefinition {
    PVI,
    PVC,
    PVT,
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

#[derive(Debug, Clone, Copy)]
pub enum DesignStandard {
    AASHTO,
    CALTRANS,
}

impl DesignStandard {
    pub fn next(self) -> Self {
        match self {
            DesignStandard::AASHTO => DesignStandard::CALTRANS,
            DesignStandard::CALTRANS => DesignStandard::AASHTO,
        }
    }
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
    pub input_sight_type: SightType,
    pub input_design_speed: String,
    pub input_design_standard: DesignStandard,
}

impl VerticalData {
    fn to_dimensions(&self) -> Result<VerticalDimensions> {
        let incoming_grade = coerce_grade(self.input_incoming_grade.clone())?;
        let outgoing_grade = coerce_grade(self.input_outgoing_grade.clone())?;
        let curve_length = coerce_elevation(self.input_length.clone())?;
        let a = (outgoing_grade-incoming_grade)/(2.0*curve_length);
        let external = a*(curve_length/2.0).powi(2);
        let design_speed = coerce_speed(self.input_design_speed.clone()).unwrap_or_default();

        Ok(VerticalDimensions { incoming_grade, outgoing_grade, curve_length, external, design_speed })
    }

    fn to_stations(&self, dimensions: &VerticalDimensions) -> Result<VerticalStations> {
        let starting_station = Station { value: coerce_station_value(self.input_station.clone())?, elevation: coerce_elevation(self.input_elevation.clone())? };
        
        match self.input_method {
            VerticalDefinition::PVC => {
                Ok(VerticalStations { 
                    pvc: starting_station, 
                    pvi: self.pvc_to_pvi(starting_station, dimensions), 
                    pvt: self.pvc_to_pvt(starting_station, dimensions), 
                })
            },
            VerticalDefinition::PVI => {
                Ok(VerticalStations { 
                    pvc: self.pvi_to_pvc(starting_station, dimensions), 
                    pvi: starting_station, 
                    pvt: self.pvi_to_pvt(starting_station, dimensions), 
                })
            },
            VerticalDefinition::PVT => {
                Ok(VerticalStations { 
                    pvc: self.pvt_to_pvc(starting_station, dimensions), 
                    pvi: self.pvt_to_pvi(starting_station, dimensions), 
                    pvt: starting_station,
                })
            },
        }   
    }

    fn pvc_to_pvi(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station { value: sts.value+dim.curve_length/2.0, elevation: sts.elevation+dim.incoming_grade*dim.curve_length/2.0 }
    }

    fn pvc_to_pvt(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station { value: sts.value+dim.curve_length, elevation: sts.elevation+dim.incoming_grade*dim.curve_length/2.0+dim.outgoing_grade*dim.curve_length/2.0 }
    }

    fn pvi_to_pvc(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station { value: sts.value-dim.curve_length/2.0, elevation: sts.elevation-dim.incoming_grade*dim.curve_length/2.0 }
    }

    fn pvi_to_pvt(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station { value: sts.value+dim.curve_length/2.0, elevation: sts.elevation+dim.outgoing_grade*dim.curve_length/2.0 }
    }

    fn pvt_to_pvc(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station { value: sts.value-dim.curve_length, elevation: sts.elevation-dim.incoming_grade*dim.curve_length/2.0-dim.outgoing_grade*dim.curve_length/2.0 }
    }

    fn pvt_to_pvi(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station { value: sts.value-dim.curve_length/2.0, elevation: sts.elevation-dim.outgoing_grade*dim.curve_length/2.0 }
    }

    pub fn to_vertical_curve(&self) -> Result<VerticalCurve> {
        let dimensions = self.to_dimensions()?;
        let stations = self.to_stations(&dimensions)?;

        Ok(VerticalCurve { dimensions, stations })
    }
}
