use std::fmt;

use crate::horizontal::*;

impl fmt::Display for HorizontalDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "~ Curve Details")?;
        writeln!(f, "Radius: {:.2}", self.radius)?;
        writeln!(f, "Curve Length: {:.2}", self.curve_length)?;
        writeln!(f, "Tangent: {:.2}", self.tangent)?;
        writeln!(f, "Long Chord: {:.2}", self.long_chord)?;
        writeln!(f, "Middle Ordinate: {:.2}", self.middle_ordinate)?;
        writeln!(f, "External: {:.2}", self.external.abs())?;
        writeln!(f, "Angle: {}", self.curve_angle)?;
        writeln!(f, "Angle/100: {}", self.curve_length_100)?;
        Ok(())
    }
}

impl fmt::Display for HorizontalStations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "~ Major Stations")?;
        writeln!(f, "PC > {:.2}", self.pc)?;
        writeln!(f, "PI > {:.2}", self.pi)?;
        writeln!(f, "PT > {:.2}", self.pt)?; 
        Ok(())
    }
}

impl fmt::Display for HorizontalCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.dimensions)?;
        writeln!(f, "{}", self.stations)?;
        Ok(())
    }
}

