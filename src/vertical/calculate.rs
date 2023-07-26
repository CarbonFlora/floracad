use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::vertical::*;
use crate::datatypes::*;

//AASHTO GREEN BOOK TABLE 3-35
lazy_static! {
    static ref AASHTO_SIGHT_TABLE: HashMap<i32, (f64, f64)> = { //stopping, crest passing, sag passing
        let mut m = HashMap::new();
        m.insert(15, (80.0, 400.0));
        m.insert(20, (115.0, 400.0));
        m.insert(25, (155.0, 450.0));
        m.insert(30, (200.0, 500.0));
        m.insert(35, (250.0, 550.0));
        m.insert(40, (305.0, 600.0));
        m.insert(45, (360.0, 700.0));
        m.insert(50, (425.0, 800.0));
        m.insert(55, (495.0, 900.0));
        m.insert(60, (570.0, 1000.0));
        m.insert(65, (645.0, 1100.0));
        m.insert(70, (730.0, 1200.0));
        m.insert(75, (820.0, 1300.0));
        m.insert(80, (910.0, 1400.0));

        m
    };
}

//CALTRANS HDM TABLE 201-1 & TABLE 201-7
lazy_static! {
    static ref HDM_SIGHT_TABLE: HashMap<i32, (f64, f64, f64)> = { //stopping, passing, decision
        let mut m = HashMap::new();
        m.insert(10, (50.0, 800.0, 450.0));
        m.insert(15, (100.0, 800.0, 450.0));
        m.insert(20, (125.0, 800.0, 450.0));
        m.insert(25, (150.0, 950.0, 450.0));
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
    pub design_speed: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalCurve {
    pub dimensions: VerticalDimensions,
    pub stations: VerticalStations,
}

impl VerticalCurve {
    pub fn calc_min_curve_length(&self, min_sight: f64, design_standard: DesignStandard, sight_type: SightType) -> Result<f64> {
        let curve_length = self.dimensions.curve_length;
        let grade_break = self.dimensions.outgoing_grade - self.dimensions.incoming_grade;
        let a = grade_break.abs()*100.0;
        if grade_break == 0.0 { // --
            return Ok(0.0)
        }
        
        match design_standard {
            DesignStandard::AASHTO => {
                match sight_type {
                    SightType::Stopping => {
                        if grade_break.is_sign_positive() { // \/
                            let l = a*min_sight.powi(2)/(400.0+3.5*min_sight);
                            if l >= min_sight {return Ok(l)}
                            let l = 2.0*min_sight-(400.0+3.5*min_sight)/a;
                            if min_sight > l {return Ok(l)}

                            return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"SAG")))
                        } else { // /\
                            let l = a*min_sight.powi(2)/2158.0;
                            if l >= min_sight {return Ok(l)}
                            let l = 2.0*min_sight-2158.0/a;
                            if min_sight > l {return Ok(l)}
                            
                            return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"CREST")))
                        }
                    },
                    SightType::Passing => {
                        if grade_break.is_sign_positive() { // \/ todo!()
                            let l = a*min_sight.powi(2)/(400.0+3.5*min_sight);
                            if l >= min_sight {return Ok(l)}
                            let l = 2.0*min_sight-(400.0+3.5*min_sight)/a;
                            if min_sight > l {return Ok(l)}

                            return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"SAG")))
                        } else { // /\
                            let l = a*min_sight.powi(2)/2800.0;
                            if l >= min_sight {return Ok(l)}
                            let l = 2.0*min_sight-2800.0/a;
                            if min_sight > l {return Ok(l)}
                            
                            return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"CREST")))
                        }
                    }
                    _ => return Err(anyhow!(format!("{:?} - {:?} hasn't been implimented.",design_standard,sight_type))),
                }

                
            },
            DesignStandard::CALTRANS => {
                match sight_type {
                    SightType::Stopping => {
                        if grade_break.is_sign_positive() { // \/
                            let l = a*min_sight.powi(2)/(400.0+3.5*min_sight);
                            if l >= min_sight {return Ok(l)}
                            let l = 2.0*min_sight-(400.0+3.5*min_sight)/a;
                            if min_sight > l {return Ok(l)}

                            return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"SAG")))
                        } else { // /\
                            let l = a*min_sight.powi(2)/1329.0;
                            if l >= min_sight {return Ok(l)}
                            let l = 2.0*min_sight-1329.0/a;
                            if min_sight > l {return Ok(l)}
                            
                            return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"CREST")))
                        }
                    },
                    SightType::Decision => {
                        if grade_break.is_sign_positive() { // \/
                            let l = a*min_sight.powi(2)/(400.0+3.5*min_sight);
                            if l >= min_sight {return Ok(l)}
                            let l = 2.0*min_sight-(400.0+3.5*min_sight)/a;
                            if min_sight > l {return Ok(l)}

                            return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"SAG")))
                        } else { // /\
                            let l = a*min_sight.powi(2)/1329.0;
                            if l >= min_sight {return Ok(l)}
                            let l = 2.0*min_sight-1329.0/a;
                            if min_sight > l {return Ok(l)}
                            
                            return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"CREST")))
                        }
                    },
                    _ => return Err(anyhow!(format!("{:?} - {:?} hasn't been implimented.",design_standard,sight_type))),
                }
            },
        };
    }

    pub fn get_min_sight(&self, design_standard: DesignStandard, sight_type: SightType) -> Option<f64> {
        match design_standard {
            DesignStandard::AASHTO => {
                let row = AASHTO_SIGHT_TABLE.get(&self.dimensions.design_speed)?;
                match sight_type {
                    SightType::Stopping => return Some(row.0),
                    SightType::Passing => return Some(row.1),
                    SightType::Decision => return None,
                };
            },
            DesignStandard::CALTRANS => {
                let row = HDM_SIGHT_TABLE.get(&self.dimensions.design_speed)?;
                match sight_type {
                    SightType::Stopping => return Some(row.0),
                    SightType::Passing => return Some(row.1),
                    SightType::Decision => return Some(row.2),
                };
            },
        };
    }

    pub fn is_compliant(&self, design_standard: DesignStandard, sight_type: SightType) -> Result<Option<(bool, f64)>> {
        let min_sight = self.get_min_sight(design_standard, sight_type);
        match min_sight {
            Some(w) => {
                let min_curve_length = self.calc_min_curve_length(w, design_standard, sight_type)?;

                Ok(Some(((self.dimensions.curve_length>=min_curve_length), min_curve_length)))
            },
            None => Err(anyhow!(format!("{:?} - {:?} doesn't contain the specified design speed.", design_standard, sight_type))),
        }

        
    }
}

    