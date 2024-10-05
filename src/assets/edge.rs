use crate::assets::Point3D;

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub start: Point3D<f64>,
    pub end: Point3D<f64>,
}

impl Edge {
    // TODO: docs
    pub fn new(start: Point3D<f64>, end: Point3D<f64>) -> Edge {
        Edge {
            start,
            end
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start)
    }
}
