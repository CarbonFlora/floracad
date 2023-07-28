use std::fmt;

use crate::vertical::*;

impl fmt::Display for VerticalDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "~ Curve Details")?;
        writeln!(f, "Curve Length: {:.2}", self.curve_length)?;
        writeln!(f, "Grade: {:.2}% -> {:.2}%", self.incoming_grade*100.0, self.outgoing_grade*100.0)?;
        writeln!(f, "External: {:.2}", self.external.abs())?;
        Ok(())
    }
}

impl fmt::Display for VerticalStations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "~ Major Stations")?;
        writeln!(f, "PVC > {:.2}", self.pvc)?;
        writeln!(f, "PVI > {:.2}", self.pvi)?;
        writeln!(f, "PVT > {:.2}", self.pvt)?; 
        Ok(())
    }
}

impl fmt::Display for VerticalCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.dimensions)?;
        writeln!(f, "{}", self.stations)?;
        Ok(())
    }
}