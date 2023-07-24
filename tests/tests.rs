#[cfg(test)]
mod tests {
    // use dms_coordinates::DMS;
    // use floracad::sight_distance::*;
    use floracad::{parse_text_file, vertical_create::VerticalCurve};
    // use floracad::angle_system::Angle;
    // use dms_coordinates::Bearing::*;
    // use floracad::horizontal_create::HorizontalCurve;
    use anyhow::Result;
    use std::fs;

// Horizontal Alignment Tests

    // #[test]
    // fn ha_generate_0() {
    //     let horizontal_alignment = HorizontalCurve::create(parse_text_file("./tests/inputs/HA/input_0.md")).expect("failed to create a horizontal curve.");
    //     assert!(true);
    //     dbg!(horizontal_alignment);
    // }

    // #[test]
    // fn ha_generate_1() {
    //     let horizontal_alignment = HorizontalCurve::create(parse_text_file("./tests/inputs/HA/input_1.md")).expect("failed to create a horizontal curve.");
    //     assert!(true);
    //     dbg!(horizontal_alignment);
    // }

    // #[test]
    // fn ha_generate_2() {
    //     let horizontal_alignment = HorizontalCurve::create(parse_text_file("./tests/inputs/HA/input_2.md")).expect("failed to create a horizontal curve.");
    //     assert!(true);
    //     dbg!(horizontal_alignment);
    // }

    // #[test]
    // fn ha_generate_3() {
    //     let horizontal_alignment = HorizontalCurve::create(parse_text_file("./tests/inputs/HA/input_3.md")).expect("failed to create a horizontal curve.");
    //     assert!(true);
    //     dbg!(horizontal_alignment);
    // }

    // #[test]
    // fn ha_generate_5() {
    //     let horizontal_alignment = HorizontalCurve::create(parse_text_file("./tests/inputs/HA/input_5.md")).expect("failed to create a horizontal curve.");
    //     assert_eq!(horizontal_alignment.stations.pt, 2006.8112637054776);
    //     dbg!(horizontal_alignment);
    // }

    // #[test]
    // fn test_dms() {
    //     let dd_value = 15.0169444444444;
    //     println!("DD: {:?}", dd_value);
    //     println!("DMS: {:?}", DMS::from_decimal_degrees(dd_value, false));
    //     assert_eq!(DMS::from_decimal_degrees(dd_value, false), DMS { degrees: 15, minutes: 1, seconds: 0.9999999998400719, bearing: East })
    // }

    // #[test]
    // fn test_to_dms() {
    //     let rad = Angle::Radians(1.0).to_dms();
    //     let dd_value = Angle::DecimalDegrees(1.4687).to_dms();
    //     assert!(matches!(rad, Angle::Dms(DMS { degrees: 57, minutes: 17, seconds: _, bearing: East })));
    //     assert!(matches!(dd_value, Angle::Dms(DMS { degrees: 1, minutes: 28, seconds: _, bearing: East })));
    //     dbg!(rad);
    //     dbg!(dd_value);
    // }

    // #[test]
    // fn test_to_dd() {
    //     let rad = Angle::Radians(1.0).to_decimal_degrees().value();
    //     let dms_1 = Angle::Dms(DMS{degrees: 15, minutes: 1, seconds: 1.0, bearing: East}).to_decimal_degrees().value();
    //     assert_eq!(rad, 57.29577951308232);
    //     assert_eq!(dms_1, 15.016944444444444);
    //     dbg!(rad);
    //     dbg!(dms_1);
    // }

    // #[test]
    // fn test_to_radians() {
    //     let dd_value = Angle::DecimalDegrees(1.0).to_radians().value();
    //     let dms_1 = Angle::Dms(DMS{degrees: 15, minutes: 1, seconds: 1.0, bearing: East}).to_radians().value();
    //     assert_eq!(dd_value, 0.017453292519943295);
    //     assert_eq!(dms_1, 0.26209512414462627);
    //     dbg!(dd_value);
    //     dbg!(dms_1);
    // }

    // #[test]
    // fn parse_table_1() {
    //     let ex1 = parse_table(SightType::Stopping);
    //     if let Ok(table) = ex1 {
    //         assert_eq!(table.len(), 15);
    //         dbg!(table);
    //     } else {
    //         println!("{:#?}", ex1);
    //         assert!(false);
    //     }
    // }

    // #[test]
    // fn min_sight_distance_1() {
    //     let stopping_table = parse_table(SightType::Stopping);
    //     if let Ok(table) = stopping_table {
    //         let ex1 = calc_min_sight_distance(table, 65, SightType::Stopping, false).expect("Failed to calculate minimum sight distance.");
    //         assert_eq!(ex1, 660.0);
    //         dbg!(ex1);
    //     } else {
    //         println!("{:#?}", stopping_table);
    //         assert!(false);
    //     }
    // }

    // #[test]
    // fn min_sight_distance_2() {
    //     let stopping_table = parse_table(SightType::Passing);
    //     if let Ok(table) = stopping_table {
    //         let ex1 = calc_min_sight_distance(table, 30, SightType::Passing, false).expect("Failed to calculate minimum sight distance.");
    //         assert_eq!(ex1, 1100.0);
    //         dbg!(ex1);
    //     } else {
    //         println!("{:#?}", stopping_table);
    //         assert!(false);
    //     }
    // }

    // #[test]
    // fn min_sight_distance_3() {
    //     let stopping_table = parse_table(SightType::Decision);
    //     if let Ok(table) = stopping_table {
    //         let ex1 = calc_min_sight_distance(table, 80, SightType::Decision, false).expect("Failed to calculate minimum sight distance.");
    //         assert_eq!(ex1, 1260.0);
    //         dbg!(ex1);
    //     } else {
    //         println!("{:#?}", stopping_table);
    //         assert!(false);
    //     }
    // }

// Vertical Alignment Tests

    #[test]
    fn va_generate_once() -> Result<()> {
        let hash = parse_text_file("./tests/inputs/VA/input_5.md")?;
        let vertical_alignment = VerticalCurve::from_hashmap(&hash);
        println!("{:#?}", vertical_alignment);
        Ok(())
    }

    #[test]
    fn va_generate_dir() -> Result<()> {
        for file in fs::read_dir("./tests/inputs/VA/")? {
            let binding = file?.path();
            let path = binding.to_str();
            if let Some(path_str) = path {
                let hash = parse_text_file(path_str)?;    
                let vertical_alignment = VerticalCurve::from_hashmap(&hash);
                println!("Path: {:?}", path);
                println!("{:#?}", vertical_alignment);
            }
            
        }
        Ok(())
    }

    // #[test]
    // fn va_generate_2() {
    //     let vertical_alignment = VerticalCurve::create(parse_text_file("./tests/inputs/VA/input_2.md")).expect("failed to create a vertical curve.");
    //     assert!(true);
    //     dbg!(vertical_alignment);
    // }

    // #[test]
    // fn va_generate_3() {
    //     let vertical_alignment = VerticalCurve::create(parse_text_file("./tests/inputs/VA/input_3.md")).expect("failed to create a vertical curve.");
    //     assert!(true);
    //     dbg!(vertical_alignment);
    // }

    // #[test]
    // fn va_generate_compare() {
    //     let vertical_alignment = VerticalCurve::create(parse_text_file("./tests/inputs/VA/input_1.md")).expect("failed to create a vertical curve.");
    //     let vertical_alignment_2 = VerticalCurve::create(parse_text_file("./tests/inputs/VA/input_2.md")).expect("failed to create a vertical curve.");
    //     let vertical_alignment_3 = VerticalCurve::create(parse_text_file("./tests/inputs/VA/input_3.md")).expect("failed to create a vertical curve.");
    //     assert!(matches!(&vertical_alignment, _vertical_alignment_2));
    //     assert!(matches!(&vertical_alignment, _vertical_alignment_3));
    //     dbg!(vertical_alignment);
    //     dbg!(vertical_alignment_2);
    //     dbg!(vertical_alignment_3);
    // }

    // #[test]
    // fn sight_dist_1() { //crest 
    //     let vertical_alignment = VerticalCurve::create(parse_text_file("./tests/inputs/VA/input_1.md")).expect("failed to create a vertical curve.");
    //     let sight = vertical_alignment.dimensions.sight_distance.unwrap();
    //     assert_eq!(sight, 17636.84210526316);
    //     dbg!(sight);
    //     dbg!(vertical_alignment);
    // }

    // #[test]
    // fn sight_dist_4() { //sag
    //     let vertical_alignment = VerticalCurve::create(parse_text_file("./tests/inputs/VA/input_4.md")).expect("failed to create a vertical curve.");
    //     let sight = vertical_alignment.dimensions.sight_distance.unwrap();
    //     assert_eq!(sight, 113.54895061791827);
    //     dbg!(sight);
    //     dbg!(vertical_alignment);
    // }
}