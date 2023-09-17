use crate::LngLatBounds;
use serde::{Deserialize, Serialize};

pub trait IntoQueryGeometry {
    fn into_query_geometry(self) -> QueryGeometry;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryGeometry {
    Point {
        lng: f64,
        lat: f64,
    },
    BBox {
        west: f64,
        south: f64,
        east: f64,
        north: f64,
    },
}

impl QueryGeometry {
    pub fn into_vec(self) -> Vec<f64> {
        match self {
            Self::Point { lng, lat } => vec![lng, lat],
            Self::BBox {
                west,
                south,
                east,
                north,
            } => vec![west, south, east, north],
        }
    }
}

impl IntoQueryGeometry for [f64; 2] {
    fn into_query_geometry(self) -> QueryGeometry {
        QueryGeometry::Point {
            lng: self[0],
            lat: self[1],
        }
    }
}

impl IntoQueryGeometry for [f64; 4] {
    fn into_query_geometry(self) -> QueryGeometry {
        QueryGeometry::BBox {
            west: self[0],
            south: self[1],
            east: self[2],
            north: self[3],
        }
    }
}

impl IntoQueryGeometry for crate::event::Point {
    fn into_query_geometry(self) -> QueryGeometry {
        QueryGeometry::Point {
            lng: self.x,
            lat: self.y,
        }
    }
}

impl IntoQueryGeometry for LngLatBounds {
    fn into_query_geometry(self) -> QueryGeometry {
        let sw = self.get_south_west();
        let ne = self.get_north_east();
        QueryGeometry::BBox {
            west: sw.lng(),
            south: sw.lat(),
            east: ne.lng(),
            north: ne.lat(),
        }
    }
}
