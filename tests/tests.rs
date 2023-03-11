#[cfg(test)]
mod tests {
    use horizontal_alignment::{*};
    use dms_coordinates::{DMS, Bearing::*};

    #[test]
    fn it_works() {
        println!("{:#?}", parse_input());
        assert!(true);
    }

    #[test]
    fn test_dms() {
        let dd_value = 15.0169444444444;
        println!("DD: {:?}", dd_value);
        println!("DMS: {:?}", DMS::from_decimal_degrees(dd_value, false));
        assert!(true);
    }

    #[test]
    fn test_to_dms() {
        let rad = Angle::Radians(1.0);
        let dd_value = Angle::DecimalDegrees(1.4687);
        println!("{:?}", rad.to_dms());
        println!("{:?}", dd_value.to_dms());
        assert!(true);
    }

    #[test]
    fn test_to_dd() {
        let rad = Angle::Radians(1.0);
        let dms_1 = Angle::Dms(DMS{degrees: 15, minutes: 1, seconds: 1.0, bearing: East});
        println!("{:?}", rad.to_decimal_degrees());
        println!("{:?}", dms_1.to_decimal_degrees());
        assert!(true);
    }

    #[test]
    fn test_to_radians() {
        let dd_value = Angle::DecimalDegrees(1.0);
        let dms_1 = Angle::Dms(DMS{degrees: 15, minutes: 1, seconds: 1.0, bearing: East});
        println!("{:?}", dd_value.to_radians());
        println!("{:?}", dms_1.to_radians());
        assert!(true);
    }

    #[test]
    fn build_ha() {
        let horizontal_alignment_1 = HorizontalCurve::create(parse_input());
        
        println!("{:#?}", parse_input());
        println!("{:#?}", horizontal_alignment_1);
        assert!(true);
    }

    #[test]
    fn parse_table_1() {
        let ex1 = parse_table(SightType::Stopping);

        println!("{:#?}", ex1);
        assert!(true);
    }

    #[test]
    fn min_sight_distance_1() {
        let stopping_table = parse_table(SightType::Stopping);
        if let Ok(stopping_table) = stopping_table {
            let ex1 = calc_min_sight_distance(stopping_table, 65, SightType::Stopping, false);
            println!("design sight distance: {:#?}", ex1);
        }
        assert!(true);
    }

    #[test]
    fn min_sight_distance_2() {
        let stopping_table = parse_table(SightType::Passing);
        if let Ok(stopping_table) = stopping_table {
            let ex1 = calc_min_sight_distance(stopping_table, 30, SightType::Passing, false);
            println!("design sight distance: {:#?}", ex1);
        }
        assert!(true);
    }

    #[test]
    fn min_sight_distance_3() {
        let stopping_table = parse_table(SightType::Decision);
        if let Ok(stopping_table) = stopping_table {
            let ex1 = calc_min_sight_distance(stopping_table, 80, SightType::Decision, false);
            println!("design sight distance: {:#?}", ex1);
        }
        assert!(true);
    } //currently I think errors should just spit out into eprintln! rather than Error or panic variants. 
}