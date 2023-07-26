use std::fmt;

use crate::vertical::{VerticalDimensions, VerticalStations, VerticalCurve, Station, CurveDetail};

impl fmt::Display for VerticalDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "~ Curve Details")?;
        writeln!(f, "Curve Length: {:.2}", self.curve_length)?;
        writeln!(f, "Grade: {:.2}% -> {:.2}%", self.incoming_grade*100.0, self.outgoing_grade*100.0)?;
        writeln!(f, "External: {:.2}", self.external.abs())?;
        // writeln!(f, "Sight Distance: {}", self.min_curve_length)?;
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

impl fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "STA: {:.0}+{:.2}, ELEV: {:.2}", (self.value/100.0).trunc(), self.value-(self.value/100.0).trunc()*100.0, self.elevation)?;
        Ok(())
    }
}

impl fmt::Display for CurveDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for station in &self.interval {
            writeln!(f, "> {:.2}", station)?;
        }
        Ok(())
    }
}