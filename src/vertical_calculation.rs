use anyhow::{Result, anyhow};


use crate::vertical_create::Station;

type StationVal = Vec<f64>;

trait StrTools {
    fn rget(&self, index: usize) -> Result<f64>;
}

impl StrTools for StationVal {
    fn rget(&self, index: usize) -> Result<f64> {
        Ok(*self.get(index).ok_or_else(|| anyhow!("Station is misconfigured."))?)
    }
}

pub fn calc_incoming_grade(g1: &str) -> Result<f64> {
    Ok(g1.parse::<f64>().map_err(|x| anyhow!("{x} cannot be converted to grade."))?)
}

pub fn calc_outgoing_grade(g2: &str) -> Result<f64> {
    Ok(g2.parse::<f64>().map_err(|x| anyhow!("{x} cannot be converted to grade."))?)
}

pub fn calc_curve_length_vertical(curve_length: &str) -> Result<f64> {
    Ok(curve_length.parse::<f64>().map_err(|x| anyhow!("{x} cannot be converted to length."))?)
}

pub fn calc_external_vertical(g1: &str, g2: &str, curve_length: &str) -> Result<f64> {
    let g1 = calc_incoming_grade(g1)?;
    let g2 = calc_outgoing_grade(g2)?;
    let curve_length = calc_curve_length_vertical(curve_length)?;
    let a = (g2-g1)/(2.0*curve_length);
    Ok(a*(curve_length/2.0).powf(2.0))
}

pub fn calc_pvc(pvi_station: &str, pvi_elevation: &str, curve_length: &str, g1: f64) -> Result<Station> {
    let pvi_station: StationVal = pvi_station.split('+').map(|x| x.parse::<f64>().unwrap_or_default()).collect();
    let pvi_value = pvi_station.rget(0)?*100.0+pvi_station.rget(1)?;
    let pvi_elevation = pvi_elevation.parse::<f64>().map_err(|x| anyhow!("{x} cannot be converted to elevation."))?;
    let curve_length = calc_curve_length_vertical(curve_length)?;
    let elevation_value = pvi_elevation-g1*curve_length/2.0;

    Ok(Station {value: pvi_value-curve_length/2.0, elevation: elevation_value})
}

pub fn calc_pvi(pvi_station: &str, pvi_elevation: &str) -> Result<Station> {
    let pvi_station: StationVal = pvi_station.split('+').map(|x| x.parse::<f64>().unwrap_or_default()).collect();
    let pvi_value = pvi_station.rget(0)?*100.0+pvi_station.rget(1)?;
    let pvi_elevation = pvi_elevation.parse::<f64>().map_err(|x| anyhow!("{x} cannot be converted to elevation."))?;

    Ok(Station {value: pvi_value, elevation: pvi_elevation})
}

pub fn calc_pvt(pvi_station: &str, pvi_elevation: &str, curve_length: &str, g2: f64) -> Result<Station> {
    let pvi_station: StationVal = pvi_station.split('+').map(|x| x.parse::<f64>().unwrap_or_default()).collect();
    let pvi_value = pvi_station.rget(0)?*100.0+pvi_station.rget(1)?;
    let pvi_elevation = pvi_elevation.parse::<f64>().map_err(|x| anyhow!("{x} cannot be converted to elevation."))?;
    let curve_length = calc_curve_length_vertical(curve_length)?;
    let elevation_value = pvi_elevation+g2*curve_length/2.0;

    Ok(Station {value: pvi_value+curve_length/2.0, elevation: elevation_value})
} 

pub fn calc_station_pvi_from_pvc(pvc_station: &str, curve_length: &str) -> Result<String> {
    let curve_length = curve_length.parse::<f64>()?;
    let pvc: StationVal = pvc_station.split('+').map(|x| x.parse::<f64>().unwrap_or_default()).collect();
    let pvc_value = pvc.rget(0)?*100.0+pvc.rget(1)?; 
    let pvi_value_left = ((pvc_value+curve_length/2.0)/100.0).trunc();
    let pvi_value_right = ((pvc_value+curve_length/2.0)/100.0).fract(); 

    Ok(pvi_value_left.to_string()+"+"+&pvi_value_right.to_string())
}

pub fn calc_station_pvi_from_pvt(pvt_station: &str, curve_length: &str) -> Result<String> {
    let curve_length = curve_length.parse::<f64>()?;
    let pvt: StationVal = pvt_station.split('+').map(|x| x.parse::<f64>().unwrap_or_default()).collect();
    let pvt_value = pvt.rget(0)?*100.0+pvt.rget(1)?;
    let pvi_value_left = ((pvt_value+curve_length/2.0)/100.0).trunc();
    let pvi_value_right = ((pvt_value+curve_length/2.0)/100.0).fract(); 

    Ok(pvi_value_left.to_string()+"+"+&pvi_value_right.to_string())
}

pub fn calc_elevation_pvi_from_pvc(pvc_elevation: &str, curve_length: &str, g1: &str) -> Result<String> {
    dbg!(&pvc_elevation);
    let pvc_elevation = pvc_elevation.parse::<f64>().map_err(|x| anyhow!("{x} cannot be converted to elevation."))?;
    let curve_length = calc_curve_length_vertical(curve_length)?;
    let g1 = g1.parse::<f64>()?;
    
    Ok((pvc_elevation+g1*curve_length/2.0).to_string())
}

pub fn calc_elevation_pvi_from_pvt(pvt_elevation: &str, curve_length: &str, g2: &str) -> Result<String> {
    let pvt_elevation = pvt_elevation.parse::<f64>().map_err(|x| anyhow!("{x} cannot be converted to elevation."))?;
    let curve_length = calc_curve_length_vertical(curve_length)?;
    let g2 = g2.parse::<f64>()?;
    
    Ok((pvt_elevation-g2*curve_length/2.0).to_string())
}
