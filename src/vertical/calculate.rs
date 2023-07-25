use anyhow::Result;

use crate::vertical::*;
use crate::datatypes::*;


#[derive(Debug, Clone, Copy)]
pub struct VerticalStations {
    pub pvc: Station, 
    pub pvi: Station,
    pub pvt: Station,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalDimensions {
    pub incoming_grade: f64,
    pub outgoing_grade: f64,
    pub curve_length: f64,
    pub external: f64,
    pub sight_distance: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalCurve {
    pub dimensions: VerticalDimensions,
    pub stations: VerticalStations,
}

impl VerticalData {
    fn to_dimensions(&self) -> Result<VerticalDimensions> {
        let incoming_grade = coerce_grade(self.input_incoming_grade.clone())?;
        let outgoing_grade = coerce_grade(self.input_outgoing_grade.clone())?;
        let curve_length = coerce_elevation(self.input_length.clone())?;
        let a = (outgoing_grade-incoming_grade)/(2.0*curve_length);
        let external = a*(curve_length/2.0).powi(2);


        Ok(VerticalDimensions { incoming_grade, outgoing_grade, curve_length, external, sight_distance: None })
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