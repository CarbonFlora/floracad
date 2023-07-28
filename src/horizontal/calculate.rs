use anyhow::{Result, anyhow};

use crate::datatypes::*;

#[derive(Debug, Clone, Copy)]
pub struct HorizontalStations {
    pub pc: Station, 
    pub pi: Station,
    pub pt: Station,
}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalDimensions {
    pub radius: f64,
    pub curve_length: f64,
    pub tangent: f64,
    pub long_chord: f64,
    pub middle_ordinate: f64,
    pub external: f64,
    pub curve_length_100: Angle, // Da
    pub curve_angle: Angle, 
    pub design_speed: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalCurve {
    pub dimensions: HorizontalDimensions,
    pub stations: HorizontalStations,
}

impl HorizontalCurve {
    pub fn calc_min_curve_length(&self, min_sight: f64, design_standard: DesignStandard, sight_type: SightType) -> Result<f64> {
        // let curve_length = self.dimensions.curve_length;
        // let grade_break = self.dimensions.outgoing_grade - self.dimensions.incoming_grade;
        // let a = grade_break.abs()*100.0;
        // if grade_break == 0.0 { // --
        //     return Ok(0.0)
        // }
        
        // match design_standard {
        //     DesignStandard::AASHTO => {
        //         match sight_type {
        //             SightType::Stopping => {
        //                 if grade_break.is_sign_positive() { // \/
        //                     let l = a*min_sight.powi(2)/(400.0+3.5*min_sight);
        //                     if l >= min_sight {return Ok(l)}
        //                     let l = 2.0*min_sight-(400.0+3.5*min_sight)/a;
        //                     if min_sight > l {return Ok(l)}

        //                     return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"SAG")))
        //                 } else { // /\
        //                     let l = a*min_sight.powi(2)/2158.0;
        //                     if l >= min_sight {return Ok(l)}
        //                     let l = 2.0*min_sight-2158.0/a;
        //                     if min_sight > l {return Ok(l)}
                            
        //                     return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"CREST")))
        //                 }
        //             },
        //             SightType::Passing => {
        //                 if grade_break.is_sign_positive() { // \/ todo!()
        //                     let l = a*min_sight.powi(2)/(400.0+3.5*min_sight);
        //                     if l >= min_sight {return Ok(l)}
        //                     let l = 2.0*min_sight-(400.0+3.5*min_sight)/a;
        //                     if min_sight > l {return Ok(l)}

        //                     return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"SAG")))
        //                 } else { // /\
        //                     let l = a*min_sight.powi(2)/2800.0;
        //                     if l >= min_sight {return Ok(l)}
        //                     let l = 2.0*min_sight-2800.0/a;
        //                     if min_sight > l {return Ok(l)}
                            
        //                     return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"CREST")))
        //                 }
        //             }
        //             _ => return Err(anyhow!(format!("{:?} - {:?} hasn't been implimented.",design_standard,sight_type))),
        //         }

                
        //     },
        //     DesignStandard::CALTRANS => {
        //         match sight_type {
        //             SightType::Stopping => {
        //                 if grade_break.is_sign_positive() { // \/
        //                     let l = a*min_sight.powi(2)/(400.0+3.5*min_sight);
        //                     if l >= min_sight {return Ok(l)}
        //                     let l = 2.0*min_sight-(400.0+3.5*min_sight)/a;
        //                     if min_sight > l {return Ok(l)}

        //                     return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"SAG")))
        //                 } else { // /\
        //                     let l = a*min_sight.powi(2)/1329.0;
        //                     if l >= min_sight {return Ok(l)}
        //                     let l = 2.0*min_sight-1329.0/a;
        //                     if min_sight > l {return Ok(l)}
                            
        //                     return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"CREST")))
        //                 }
        //             },
        //             SightType::Decision => {
        //                 if grade_break.is_sign_positive() { // \/
        //                     let l = a*min_sight.powi(2)/(400.0+3.5*min_sight);
        //                     if l >= min_sight {return Ok(l)}
        //                     let l = 2.0*min_sight-(400.0+3.5*min_sight)/a;
        //                     if min_sight > l {return Ok(l)}

        //                     return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"SAG")))
        //                 } else { // /\
        //                     let l = a*min_sight.powi(2)/1329.0;
        //                     if l >= min_sight {return Ok(l)}
        //                     let l = 2.0*min_sight-1329.0/a;
        //                     if min_sight > l {return Ok(l)}
                            
        //                     return Err(anyhow!(format!("Failed at: {:?} - {:?} - {}",design_standard,sight_type,"CREST")))
        //                 }
        //             },
        //             _ => return Err(anyhow!(format!("{:?} - {:?} hasn't been implimented.",design_standard,sight_type))),
        //         }
        //     },
        // };
        todo!()
    }

    pub fn get_min_sight(&self, design_standard: DesignStandard, sight_type: SightType) -> Option<f64> {
        // match design_standard {
        //     DesignStandard::AASHTO => {
        //         let row = AASHTO_SIGHT_TABLE.get(&self.dimensions.design_speed)?;
        //         match sight_type {
        //             SightType::Stopping => return Some(row.0),
        //             SightType::Passing => return Some(row.1),
        //             SightType::Decision => return None,
        //         };
        //     },
        //     DesignStandard::CALTRANS => {
        //         let row = HDM_SIGHT_TABLE.get(&self.dimensions.design_speed)?;
        //         match sight_type {
        //             SightType::Stopping => return Some(row.0),
        //             SightType::Passing => return Some(row.1),
        //             SightType::Decision => return Some(row.2),
        //         };
        //     },
        // };
        todo!()
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