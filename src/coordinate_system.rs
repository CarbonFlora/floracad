use geo::{Coord, Polygon};
//use serde::{Serialize, Deserialize};

struct CrossSection {
    components: Vec<Polygon>,
    origin: Coord,
}

impl CrossSection {
    pub fn from(origin: Coord) -> CrossSection {
        CrossSection { components: Vec::new(), origin }
    }

    pub fn add_polygon(mut self, polygon: Polygon) -> Self {
        self.components.push(polygon);

        self
    }

    //use serde to be able to save multipolygons into json or smthing.
    //https://github.com/serde-rs/json
}