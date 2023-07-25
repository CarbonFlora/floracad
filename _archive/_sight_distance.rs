// use std::collections::HashMap;
// use std::io::{BufReader, BufRead, Error};
// use std::fs::File;

// use crate::horizontal_create::HorizontalCurve;
// use crate::vertical_create::VerticalCurve;

// #[derive(Debug, Clone, Copy)]
// pub enum SightType {
//     Stopping,
//     Passing,
//     Decision,
// }

// #[derive(Debug)]
// pub enum Curve {
//     HorizontalCurve (HorizontalCurve),
//     VerticalCurve (VerticalCurve),
// }

// //once per table type of deal at program startup?
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
//     Ok(arguments)
// }

// //The stopping sight distances in Table 201.1 should be increased by 20 percent on sustained downgrades steeper than 3 percent and longer than one mile. use figure 201.6
// pub fn calc_min_sight_distance(table: HashMap<i32, Vec<f64>>, design_speed: i32, sight_type: SightType, sustained_downgrade: bool) -> Result<f64, Error> {
//     let mut minimum_sight_distance = match sight_type {
//         SightType::Stopping => table.get(&design_speed).expect("design speed isn't in table.")[0],
//         SightType::Passing => table.get(&design_speed).expect("design speed isn't in table.")[1],
//         SightType::Decision => table.get(&design_speed).expect("design speed isn't in table.")[0],
//     };
//     if sustained_downgrade { //note: this should only apply to stopping sight type.
//         minimum_sight_distance *= 1.2;
//     }
//     Ok(minimum_sight_distance)
// }

// impl Curve {
//     pub fn examine_functional<'a>(&self, sight_type: SightType, design_speed: i32, sustained_downgrade: bool) -> HashMap<&'a str, bool> {
//         let mut tests = HashMap::new();
//         tests.insert("sight_distance", self.is_within_minimum_sight_distance(sight_type, design_speed, sustained_downgrade));
        
//         tests
//     }
    
//     fn is_within_minimum_sight_distance(&self, sight_type: SightType, design_speed: i32, sustained_downgrade: bool) -> bool {
//         let table = parse_table(sight_type).expect("table borked.");
//         if let Ok(sight_dist_min) = calc_min_sight_distance(table, design_speed, sight_type, sustained_downgrade) {
//             dbg!(sight_dist_min);
//             match self {
//                 Curve::HorizontalCurve(a) => {
//                     if let Some(sight_dist_actual) = a.dimensions.sight_distance {
//                         if a.dimensions.curve_length < sight_dist_actual {
//                             println!("curve length is greater than sight distance calculations.");
//                             return true;
//                         } else if sight_dist_actual >= sight_dist_min {
//                             return true;
//                         } 
//                     }
//                 },
//                 Curve::VerticalCurve(a) => {
//                     if let Some(sight_dist_actual) = a.dimensions.sight_distance {
//                         if sight_dist_actual >= sight_dist_min {
//                             return true;
//                         }
//                     }
//                 },
//             }
//         }
//         false
//     }
// }

// // impl Curve for VerticalCurve {

// // }
