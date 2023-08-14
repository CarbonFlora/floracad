#[cfg(test)]
mod hori_tests {
    use floracad::horizontal::{
        HorizontalBuildDefinition, HorizontalData, HorizontalStationDefinition,
    };

    #[test]
    fn h1() {
        let horizontal_data = HorizontalData {
            input_station_method: HorizontalStationDefinition::PI,
            input_build_method: HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "10284+50".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "63d15\'34\"".to_string(),
            input_design_speed: "65".to_string(),
            input_m: "1000".to_string(),
            ..Default::default()
        };
        let hori_angle = horizontal_data.to_horizontal_curve();
        match hori_angle {
            Ok(w) => println!("O: {:#?}", w),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn h2() {
        let horizontal_data = HorizontalData {
            input_station_method: HorizontalStationDefinition::PC,
            input_build_method: HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "100+00".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "63d15\'34\"".to_string(),
            input_design_speed: "65".to_string(),
            input_m: "1000".to_string(),
            ..Default::default()
        };
        let hori_angle = horizontal_data.to_horizontal_curve();
        match hori_angle {
            Ok(w) => println!("O: {:#?}", w),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn h3() {
        let horizontal_data = HorizontalData {
            input_station_method: HorizontalStationDefinition::PC,
            input_build_method: HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "100+00".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "180".to_string(),
            ..Default::default()
        };
        let hori_angle = horizontal_data.to_horizontal_curve();
        match hori_angle {
            Ok(w) => println!("O: {:#?}", w),
            Err(e) => println!("{}", e),
        }
    }
}
