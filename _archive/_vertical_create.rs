

// impl VerticalCurve {
    
    // fn define_vertical_curve(raw_hashmap: &HashMap<String, String>) -> Result<VerticalDefinition> {
    //     match raw_hashmap {
    //         x if x.contains_key("PVI-st") && x.contains_key("PVI-elev") => Ok(VerticalDefinition::PVI),
    //         x if x.contains_key("PVC-st") && x.contains_key("PVC-elev") => Ok(VerticalDefinition::PVC),
    //         x if x.contains_key("PVT-st") && x.contains_key("PVT-elev") => Ok(VerticalDefinition::PVT),
    //         _ => Err(anyhow!(format!("Missing information to define a vertical curve.\nEx. PVI-st=8+90.33\n    PVI-elev=138.61"))),
    //     }
    // }

    // pub fn from_hashmap(raw_hashmap: &HashMap<String, String>) -> Result<VerticalCurve> {
    //     let incoming_grade = raw_hashmap.get("inc").ok_or_else(|| anyhow!("Missing incoming grade."))?;
    //     let outgoing_grade = raw_hashmap.get("out").ok_or_else(|| anyhow!("Missing outgoing grade."))?;
    //     let curve_length = raw_hashmap.get("length").ok_or_else(|| anyhow!("Missing length."))?;
    //     let pvi_station;
    //     let pvi_elevation;
    //     let vertical_definition = Self::define_vertical_curve(&raw_hashmap)?;
    //     match vertical_definition {
    //         VerticalDefinition::PVI => {
    //             pvi_station = raw_hashmap.get("PVI-st").unwrap().clone(); 
    //             pvi_elevation = raw_hashmap.get("PVI-elev").unwrap().clone()},
    //         VerticalDefinition::PVC => {
    //             pvi_station = calc_station_pvi_from_pvc(raw_hashmap.get("PVC-st").unwrap(), curve_length)?; 
    //             pvi_elevation = calc_elevation_pvi_from_pvc(raw_hashmap.get("PVC-elev").unwrap(), curve_length, incoming_grade)?},
    //         VerticalDefinition::PVT => {
    //             pvi_station = calc_station_pvi_from_pvt(raw_hashmap.get("PVT-st").unwrap(), curve_length)?; 
    //             pvi_elevation = calc_elevation_pvi_from_pvt(raw_hashmap.get("PVT-elev").unwrap(), curve_length, outgoing_grade)?},
    //     }

    //     let mut curve = VerticalCurve {
    //         dimensions: VerticalDimensions { 
    //             incoming_grade: calc_incoming_grade(incoming_grade)?, 
    //             outgoing_grade: calc_outgoing_grade(outgoing_grade)?,
    //             curve_length: calc_curve_length_vertical(curve_length)?, 
    //             external: calc_external_vertical(incoming_grade, outgoing_grade, curve_length)?, 
    //             sight_distance: None,
    //         }, 
    //         stations: VerticalStations { 
    //             pvc: calc_pvc(&pvi_station, &pvi_elevation, curve_length, calc_incoming_grade(incoming_grade)?)?, 
    //             pvi: calc_pvi(&pvi_station, &pvi_elevation)?, 
    //             pvt: calc_pvt(&pvi_station, &pvi_elevation, curve_length, calc_outgoing_grade(outgoing_grade)?)?,
    //         }
    //     };
    //     curve.dimensions.sight_distance = Some(curve.calc_sight_distance());

    //     if Curve::VerticalCurve(curve).examine_functional(SightType::Stopping, 65, false).values().all(|b| *b) {
    //         println!("Curve passes all relevant inspections.");
    //     } else {
    //         println!("Curve fails all relevant inspections.");
    //     }

    //     Ok(curve)
    // }

    //from HDM, assuming 3.5 ft driver eye height, 0.5ft obstruction height.
//     fn calc_sight_distance(&self) -> f64 { //untested! todo!()
//         let grade_diff = (self.dimensions.outgoing_grade-self.dimensions.incoming_grade).abs();
//         let curve_length = self.dimensions.curve_length;
        
//         if self.dimensions.incoming_grade > self.dimensions.outgoing_grade { //crest curve handling
//             let eq_sight_1 = ((grade_diff*curve_length+1329.0)/(2.0*grade_diff)).abs(); //fails on no grade difference.
//             let eq_sight_2 = (1329.0f64.sqrt()*curve_length.sqrt()/grade_diff.sqrt()).abs(); //fails on no grade difference.
//             //dbg!(&eq_sight_1);
//             //dbg!(&eq_sight_2);
//             if eq_sight_1 > curve_length {
//                 return eq_sight_1;
//             } else if eq_sight_2 < curve_length {
//                 return eq_sight_2;
//             } else {
//                 panic!("vertical curve crest curve handling failed.");
//             }
//         } else if self.dimensions.incoming_grade < self.dimensions.outgoing_grade {
//             let eq_sight_1 = ((2.0*(grade_diff*curve_length+400.0))/(4.0*grade_diff-7.0)).abs();
//             let eq_sight_2 = ((1600.0*curve_length.sqrt())/((6400.0*grade_diff+49.0*curve_length).sqrt()+7.0*curve_length.sqrt())).abs();
//             //dbg!(&eq_sight_1);
//             //dbg!(&eq_sight_2);
//             if eq_sight_1 > curve_length {
//                 return eq_sight_1;
//             } else if eq_sight_2 < curve_length {
//                 return eq_sight_2;
//             } else {
//                 panic!("vertical curve sag curve handling failed.");
//             }
//         } else {
//             panic!("failed stopping sight distance calculations.");
//         }
//     }
// }