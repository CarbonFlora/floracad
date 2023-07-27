use anyhow::Result;

use crate::datatypes::*;

pub mod calculate;
// pub mod interval;
// pub mod display;

use self::calculate::*;
// use self::interval::*;


#[derive(Debug, Clone, Copy)]
pub enum HorizontalDefinition {
    PI,
    PC,
    PT,
}

impl HorizontalDefinition {
    pub fn next(self) -> Self {
        match self {
            HorizontalDefinition::PC => HorizontalDefinition::PI,
            HorizontalDefinition::PI => HorizontalDefinition::PT,
            HorizontalDefinition::PT => HorizontalDefinition::PC,
        }
    } 
}

#[derive(Debug, Clone)]
pub struct HorizontalData {
    pub input_method: HorizontalDefinition,
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

impl HorizontalData {
    fn to_dimensions(&self) -> Result<HorizontalDimensions> {
        let curve_length = coerce_elevation(self.input_length.clone())?;
        let design_speed = coerce_speed(self.input_design_speed.clone()).unwrap_or_default();

        todo!();
        // Ok(HorizontalDimensions { 
        //     radius: (), 
        //     curve_length, 
        //     tangent: (), 
        //     long_chord: (), 
        //     middle_ordinate: (), 
        //     external: (), 
        //     curve_length_100: (), 
        //     curve_angle: (), 
        //     design_speed: () 
        // })
    }

    fn to_stations(&self, dimensions: &HorizontalDimensions) -> Result<HorizontalStations> {
        let starting_station = Station { value: coerce_station_value(self.input_station.clone())?, elevation: coerce_elevation(self.input_elevation.clone())? };
        
        match self.input_method {
            HorizontalDefinition::PC => {
                Ok(HorizontalStations { 
                    pvc: starting_station, 
                    pvi: self.pvc_to_pvi(starting_station, dimensions), 
                    pvt: self.pvc_to_pvt(starting_station, dimensions), 
                })
            },
            HorizontalDefinition::PI => {
                Ok(HorizontalStations { 
                    pvc: self.pvi_to_pvc(starting_station, dimensions), 
                    pvi: starting_station, 
                    pvt: self.pvi_to_pvt(starting_station, dimensions), 
                })
            },
            HorizontalDefinition::PT => {
                Ok(HorizontalStations { 
                    pvc: self.pvt_to_pvc(starting_station, dimensions), 
                    pvi: self.pvt_to_pvi(starting_station, dimensions), 
                    pvt: starting_station,
                })
            },
        }   
    }

    fn pvc_to_pvi(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        todo!();
        // Station { value: sts.value+dim.curve_length/2.0, elevation: sts.elevation+dim.incoming_grade*dim.curve_length/2.0 }
    }

    fn pvc_to_pvt(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        todo!();
        // Station { value: sts.value+dim.curve_length, elevation: sts.elevation+dim.incoming_grade*dim.curve_length/2.0+dim.outgoing_grade*dim.curve_length/2.0 }
    }

    fn pvi_to_pvc(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        todo!();
        // Station { value: sts.value-dim.curve_length/2.0, elevation: sts.elevation-dim.incoming_grade*dim.curve_length/2.0 }
    }

    fn pvi_to_pvt(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        todo!();
        // Station { value: sts.value+dim.curve_length/2.0, elevation: sts.elevation+dim.outgoing_grade*dim.curve_length/2.0 }
    }

    fn pvt_to_pvc(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        todo!();
        // Station { value: sts.value-dim.curve_length, elevation: sts.elevation-dim.incoming_grade*dim.curve_length/2.0-dim.outgoing_grade*dim.curve_length/2.0 }
    }

    fn pvt_to_pvi(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        todo!();
        // Station { value: sts.value-dim.curve_length/2.0, elevation: sts.elevation-dim.outgoing_grade*dim.curve_length/2.0 }
    }

    pub fn to_horizontal_curve(&self) -> Result<HorizontalCurve> {
        let dimensions = self.to_dimensions()?;
        let stations = self.to_stations(&dimensions)?;

        Ok(HorizontalCurve { dimensions, stations })
    }
}
