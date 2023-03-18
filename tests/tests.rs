#[cfg(test)]
mod tests {
    use dms_coordinates::{DMS};
    use horizontal_alignment::{parse_input, vertical_create::VerticalCurve};
    use horizontal_alignment::angle_system::Angle;
    use horizontal_alignment::horizontal_create::SightType;
    use dms_coordinates::Bearing::*;
    use horizontal_alignment::horizontal_create::{HorizontalCurve, parse_table, calc_min_sight_distance};

// Horizontal Alignment Tests

    #[test]
    fn ha_generate_1() {
        let horizontal_alignment = HorizontalCurve::create(parse_input("./tests/inputs/HA/input_1.md")).expect("failed to create a horizontal curve.");
        assert!(true);
        dbg!(horizontal_alignment);
    }

    #[test]
    fn ha_generate_2() {
        let horizontal_alignment = HorizontalCurve::create(parse_input("./tests/inputs/HA/input_2.md")).expect("failed to create a horizontal curve.");
        assert!(true);
        dbg!(horizontal_alignment);
    }

    #[test]
    fn ha_generate_3() {
        let horizontal_alignment = HorizontalCurve::create(parse_input("./tests/inputs/HA/input_3.md")).expect("failed to create a horizontal curve.");
        assert!(true);
        dbg!(horizontal_alignment);
    }

    #[test]
    fn test_dms() {
        let dd_value = 15.0169444444444;
        println!("DD: {:?}", dd_value);
        println!("DMS: {:?}", DMS::from_decimal_degrees(dd_value, false));
        assert_eq!(DMS::from_decimal_degrees(dd_value, false), DMS { degrees: 15, minutes: 1, seconds: 0.9999999998400719, bearing: East })
    }

    #[test]
    fn test_to_dms() {
        let rad = Angle::Radians(1.0).to_dms();
        let dd_value = Angle::DecimalDegrees(1.4687).to_dms();
        assert!(matches!(rad, Angle::Dms(DMS { degrees: 57, minutes: 17, seconds: _, bearing: East })));
        assert!(matches!(dd_value, Angle::Dms(DMS { degrees: 1, minutes: 28, seconds: _, bearing: East })));
        dbg!(rad);
        dbg!(dd_value);
    }

    #[test]
    fn test_to_dd() {
        let rad = Angle::Radians(1.0).to_decimal_degrees().value();
        let dms_1 = Angle::Dms(DMS{degrees: 15, minutes: 1, seconds: 1.0, bearing: East}).to_decimal_degrees().value();
        assert_eq!(rad, 57.29577951308232);
        assert_eq!(dms_1, 15.016944444444444);
        dbg!(rad);
        dbg!(dms_1);
    }

    #[test]
    fn test_to_radians() {
        let dd_value = Angle::DecimalDegrees(1.0).to_radians().value();
        let dms_1 = Angle::Dms(DMS{degrees: 15, minutes: 1, seconds: 1.0, bearing: East}).to_radians().value();
        assert_eq!(dd_value, 0.017453292519943295);
        assert_eq!(dms_1, 0.26209512414462627);
        dbg!(dd_value);
        dbg!(dms_1);
    }

    #[test]
    fn parse_table_1() {
        let ex1 = parse_table(SightType::Stopping);
        if let Ok(table) = ex1 {
            assert_eq!(table.len(), 15);
            dbg!(table);
        } else {
            println!("{:#?}", ex1);
            assert!(false);
        }
    }

    #[test]
    fn min_sight_distance_1() {
        let stopping_table = parse_table(SightType::Stopping);
        if let Ok(table) = stopping_table {
            let ex1 = calc_min_sight_distance(table, 65, SightType::Stopping, false).expect("Failed to calculate minimum sight distance.");
            assert_eq!(ex1, 660.0);
            dbg!(ex1);
        } else {
            println!("{:#?}", stopping_table);
            assert!(false);
        }
    }

    #[test]
    fn min_sight_distance_2() {
        let stopping_table = parse_table(SightType::Passing);
        if let Ok(table) = stopping_table {
            let ex1 = calc_min_sight_distance(table, 30, SightType::Passing, false).expect("Failed to calculate minimum sight distance.");
            assert_eq!(ex1, 1100.0);
            dbg!(ex1);
        } else {
            println!("{:#?}", stopping_table);
            assert!(false);
        }
    }

    #[test]
    fn min_sight_distance_3() {
        let stopping_table = parse_table(SightType::Decision);
        if let Ok(table) = stopping_table {
            let ex1 = calc_min_sight_distance(table, 80, SightType::Decision, false).expect("Failed to calculate minimum sight distance.");
            assert_eq!(ex1, 1260.0);
            dbg!(ex1);
        } else {
            println!("{:#?}", stopping_table);
            assert!(false);
        }
    }

// Vertical Alignment Tests

    #[test]
    fn va_generate() {
        let vertical_alignment = VerticalCurve::create(parse_input("./tests/inputs/VA/input_1.md")).expect("failed to create a vertical curve.");
        assert!(true);
        dbg!(vertical_alignment);
    }

}