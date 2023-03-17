use std::collections::HashMap;
use std::io::{BufReader, BufRead, Error};
use std::fs::File;
use crate::angle_system::Angle;
use crate::vertical_calculation::*;

#[derive(Debug)]
pub struct VerticalCurve {
    dimensions: VerticalDimensions,
    stations: VerticalStations,
}

#[derive(Debug)]
pub struct Station {
    pub value: f64,
    pub elevation: f64,
}

impl Station {
    
}

#[derive(Debug)]
pub struct VerticalStations {
    pvc: Station, 
    pvi: Station,
    pvt: Station,
}

#[derive(Debug)]
pub struct VerticalDimensions {
    incoming_grade: f64,
    outgoing_grade: f64,
    curve_length: f64,
    external: f64,
    long_chord: f64,
    sight_distance: Option<f64>,
}

impl VerticalCurve {
    pub fn create(pre_given: Result<HashMap<String, String>, Error>) -> Result<VerticalCurve, Error> { //http://www.sd-w.com/channel_flow/vertical_curves/
        let pre_given = pre_given?;
        let given = pre_given;
        //let given = HorizontalCurve::nudge_create(&mut pre_given); //todo!()

        let pvi_station = given.get("PVI-st").unwrap();
        let pvi_elevation = given.get("PVI-elev").unwrap();
        let incoming_grade = given.get("inc").unwrap();
        let outgoing_grade = given.get("out").unwrap();
        let curve_length = given.get("length").unwrap();

        let dimensions = VerticalDimensions { 
            incoming_grade: calc_incoming_grade(incoming_grade),
            outgoing_grade: calc_outgoing_grade(outgoing_grade),
            curve_length: calc_curve_length_vertical(curve_length),
            external: calc_external_vertical(incoming_grade, outgoing_grade, curve_length),
            long_chord: calc_long_chord_vertical(), //todo!()
            sight_distance: None,
        };
        let stations = VerticalStations { 
            pvc: calc_pvc(pvi_station, pvi_elevation, curve_length, dimensions.incoming_grade), //pvc = pvi - curve_length/2
            pvi: calc_pvi(pvi_station, pvi_elevation),  
            pvt: calc_pvt(pvi_station, pvi_elevation, curve_length, dimensions.outgoing_grade), //pvt = pvc + curve_length
        };

        Ok(VerticalCurve {dimensions, stations})
    }

    // pub fn nudge_create(given: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
    //     if !given.contains_key("Da") {
    //         given.insert("Da".to_string(), radius_to_da(given.get("R").expect("missing R (radius) or Da")));
    //     }
    //     if !given.contains_key("PI") { //given R, Da, I
    //         if given.contains_key("PC") {
    //             let value = calc_pi_from_pc(given.get("I").unwrap(), given.get("R").unwrap(), given.get("PC").unwrap());

    //             given.insert("PI".to_string(), value);
    //         } else if given.contains_key("PT") {
    //             let value = calc_pi_from_pt(given.get("I").unwrap(), given.get("R").unwrap(), given.get("PT").unwrap());

    //             given.insert("PI".to_string(), value);
    //         } else {
    //             panic!("input doesn't contain PC, PI, PT");   
    //         }
    //     }
    //
    //    given
    //}
}

#[derive(Debug)]
pub enum SightType {
    Stopping,
    Passing,
    Decision,
}

// pub fn parse_table(sight_type: SightType) -> Result<HashMap<i32, Vec<f64>>, Error> {
//     let buffered;
//     match sight_type {
//         SightType::Stopping => buffered = BufReader::new(File::open("look_up/CALTRANS_HDM/table_201-1.txt")?),
//         SightType::Passing => buffered = BufReader::new(File::open("look_up/CALTRANS_HDM/table_201-1.txt")?),
//         SightType::Decision => buffered = BufReader::new(File::open("look_up/CALTRANS_HDM/table_201-7.txt")?),
//     };
//     let mut arguments = HashMap::new();

//     for line in buffered.lines().flatten() {
//         if let Some(first_number) = line.split_whitespace().next() {
//             if let Ok(num) = first_number.to_string().parse::<i32>() {
//                 //println!("{:?}, {:#?}", num, line.split_whitespace().collect::<Vec<&str>>());
//                 arguments.insert(num, line.split_whitespace().skip(1)
//                 .map(|x| x.parse::<f64>().expect("Table configured improperly. Remove commas from #s.")).collect::<Vec<f64>>());
//             }
//         }
//     }
//     //arguments.insert(2, [32.0,36.0]);
//     Ok(arguments)
// }

//The stopping sight distances in Table 201.1 should be increased by 20 percent on sustained downgrades steeper than 3 percent and longer than one mile. use figure 201.6
// pub fn calc_min_sight_distance(table: HashMap<i32, Vec<f64>>, design_speed: i32, sight_type: SightType, sustained_downgrade: bool) -> Result<f64, Error> {
//     let mut minimum_sight_distance = match sight_type {
//         SightType::Stopping => table.get(&design_speed).expect("design speed isn't in table.")[0],
//         SightType::Passing => table.get(&design_speed).expect("design speed isn't in table.")[1],
//         SightType::Decision => table.get(&design_speed).expect("design speed isn't in table.")[0],
//     };
//     if sustained_downgrade {
//         minimum_sight_distance *= 1.2;
//     }
//     Ok(minimum_sight_distance)
// }