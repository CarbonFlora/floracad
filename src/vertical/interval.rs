use std::fmt;

// use anyhow::Result;

// use crate::vertical::*;

use super::calculate::VerticalCurve;

impl fmt::Display for VerticalCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Curve Length: {}", self.dimensions.curve_length)?;
        Ok(())
    }
}

impl VerticalCurve {
    
}