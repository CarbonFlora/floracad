// use anyhow::{Result, anyhow};
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::datatypes::{DesignStandard, SightType};

//AASHTO GREEN BOOK TABLE 3-35
lazy_static! {
    static ref AASHTO_SIGHT_TABLE: HashMap<i32, (f64, f64)> = { //stopping, crest passing, sag passing
        let mut m = HashMap::new();
        m.insert(15, (80.0, 400.0));
        m.insert(20, (115.0, 400.0));
        m.insert(25, (155.0, 450.0));
        m.insert(30, (200.0, 500.0));
        m.insert(35, (250.0, 550.0));
        m.insert(40, (305.0, 600.0));
        m.insert(45, (360.0, 700.0));
        m.insert(50, (425.0, 800.0));
        m.insert(55, (495.0, 900.0));
        m.insert(60, (570.0, 1000.0));
        m.insert(65, (645.0, 1100.0));
        m.insert(70, (730.0, 1200.0));
        m.insert(75, (820.0, 1300.0));
        m.insert(80, (910.0, 1400.0));

        m
    };
}

//CALTRANS HDM TABLE 201-1 & TABLE 201-7 (2020 7th ed.)
lazy_static! {
    static ref HDM_SIGHT_TABLE: HashMap<i32, (f64, f64, f64)> = { //stopping, passing, decision
        let mut m = HashMap::new();
        m.insert(10, (50.0, 800.0, 450.0));
        m.insert(15, (100.0, 800.0, 450.0));
        m.insert(20, (125.0, 800.0, 450.0));
        m.insert(25, (150.0, 950.0, 450.0));
        m.insert(30, (200.0, 1100.0, 450.0));
        m.insert(35, (250.0, 1300.0, 525.0));
        m.insert(40, (300.0, 1500.0, 600.0));
        m.insert(45, (360.0, 1650.0, 675.0));
        m.insert(50, (430.0, 1800.0, 750.0));
        m.insert(55, (500.0, 1950.0, 865.0));
        m.insert(60, (580.0, 2100.0, 990.0));
        m.insert(65, (660.0, 2300.0, 1050.0));
        m.insert(70, (750.0, 2500.0, 1105.0));
        m.insert(75, (840.0, 2600.0, 1180.0));
        m.insert(80, (930.0, 2700.0, 1260.0));

        m
    };
}

pub fn get_min_sight(
    design_speed: i32,
    design_standard: DesignStandard,
    sight_type: SightType,
) -> Option<f64> {
    match design_standard {
        DesignStandard::AASHTO => {
            let row = AASHTO_SIGHT_TABLE.get(&design_speed)?;
            match sight_type {
                SightType::Stopping => Some(row.0),
                SightType::Passing => Some(row.1),
                SightType::Decision => None,
            }
        }
        DesignStandard::CALTRANS => {
            let row = HDM_SIGHT_TABLE.get(&design_speed)?;
            match sight_type {
                SightType::Stopping => Some(row.0),
                SightType::Passing => Some(row.1),
                SightType::Decision => Some(row.2),
            }
        }
    }
}
