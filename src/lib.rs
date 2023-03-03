struct HorizontalCurve {

}

struct HorizontalCriticalStations {
    pc: i32,
    pi: i32,
    pt: i32,

}

struct HorizontalDimensions {
    radius: i32,
    curve_length: i32,
    tangent_distance: i32,
    long_chord: i32,
    middle_ordinate: i32,
    external: i32,
    curve_length_100: i32,
    curve_angle: angle,
}

struct angle {
    degrees: i32,
    radians: i32,
}

impl angle {
    fn init (&mut self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
