pub mod calculate;
pub mod interval;

#[derive(Debug, Clone, Copy)]
pub enum VerticalDefinition {
    PVI,
    PVC,
    PVT,
}

#[derive(Debug, Clone)]
pub struct VerticalData {
    pub input_method: VerticalDefinition,
    pub input_station: String,
    pub input_elevation: String,
    pub input_incoming_grade: String,
    pub input_outgoing_grade: String,
    pub input_length: String,
    pub input_station_interval: String,
}

impl VerticalDefinition {
    pub fn next(self) -> Self {
        match self {
            VerticalDefinition::PVC => VerticalDefinition::PVI,
            VerticalDefinition::PVI => VerticalDefinition::PVT,
            VerticalDefinition::PVT => VerticalDefinition::PVC,
        }
    } 
}