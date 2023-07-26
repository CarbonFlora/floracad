use anyhow::Result;
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::vertical::*;
use crate::datatypes::*;

//CALTRANS HDM TABLE 201-1 & TABLE 201-7
lazy_static! {
    static ref SIGHT_TABLE: HashMap<i32, (f64, f64, f64)> = { //stopping, passing, decision
        let mut m = HashMap::new();
        m.insert(10, (50.0, 0.0, 0.0));
        m.insert(15, (100.0, 0.0, 0.0));
        m.insert(20, (125.0, 800.0, 0.0));
        m.insert(25, (150.0, 950.0, 0.0));
        m.insert(30, (200.0, 1100.0, 450.0));
        m.insert(35, (250.0, 1300.0, 525.0));
        m.insert(40, (300.0, 1500.0, 600.0));
        m.insert(45, (360.0, 1650.0, 675.0));
        m.insert(50, (430.0, 1800.0, 750.0));
        m.insert(55, (500.0, 1950.0, 865.0));
        m.insert(60, (580.0, 2100.0, 990.0));
        m.insert(65, (660.0, 2300.0, 1050.0));
        m.insert(70, (750.0, 2500.0, 1105.0));
        m.insert(75, (840.0, 2600.0, 1180.0));
        m.insert(80, (930.0, 2700.0, 1260.0));

        m
    };
}

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
    pub sight_distance: f64,
    pub design_speed: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalCurve {
    pub dimensions: VerticalDimensions,
    pub stations: VerticalStations,
}

impl VerticalData {
    pub fn calc_sight_distance(&self, incoming_grade: f64, outgoing_grade: f64, curve_length: f64) -> f64 {
        let grade_break = outgoing_grade - incoming_grade;
        let a = grade_break.abs()*100.0;
        
        if grade_break == 0.0 { // --
            264000.0
        } else if grade_break > 0.0 { // \/
            let s = (-228.571*curve_length.sqrt())/(curve_length.sqrt()-0.142857*(6400.0*a+49.0*curve_length).sqrt());
            if curve_length >= s {
                return s
            }

            let s = 2.0*(a*curve_length+400.0)/(4.0*a-7.0);
            if s > curve_length {
                return s
            }
            return 0.0
        } else { // /\
            let s = (curve_length*1329.0/a).sqrt();
            if curve_length >= s {
                return s
            }

            let s = (curve_length + 1329.0/a)/2.0;
            if s > curve_length {
                return s
            }
            return 0.0
        }
    }

    fn to_dimensions(&self) -> Result<VerticalDimensions> {
        let incoming_grade = coerce_grade(self.input_incoming_grade.clone())?;
        let outgoing_grade = coerce_grade(self.input_outgoing_grade.clone())?;
        let curve_length = coerce_elevation(self.input_length.clone())?;
        let a = (outgoing_grade-incoming_grade)/(2.0*curve_length);
        let external = a*(curve_length/2.0).powi(2);
        let design_speed = coerce_speed(self.input_design_speed.clone()).unwrap_or_default();
        let sight_distance = self.calc_sight_distance(incoming_grade, outgoing_grade, curve_length);

        Ok(VerticalDimensions { incoming_grade, outgoing_grade, curve_length, external, sight_distance, design_speed })
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

impl VerticalCurve {
    pub fn is_compliant(&self, sight_type: SightType, speed: i32) -> Option<(bool, f64)> {
        match sight_type {
            SightType::Stopping => return Some((self.dimensions.sight_distance >= SIGHT_TABLE.get(&speed)?.0, SIGHT_TABLE.get(&speed)?.0)),
            SightType::Passing => return Some((self.dimensions.sight_distance >= SIGHT_TABLE.get(&speed)?.1, SIGHT_TABLE.get(&speed)?.1)),
            SightType::Decision => return Some((self.dimensions.sight_distance >= SIGHT_TABLE.get(&speed)?.2, SIGHT_TABLE.get(&speed)?.2)),
        }
    }
}