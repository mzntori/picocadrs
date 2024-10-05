use crate::assets::Point3D;

/// Edge of a mesh.
/// This is not actually stored in the project file, however it can be constructed from it.
///
/// Every edge has a start and end point, however these points are treated the same.
/// This means an edge going from `0, 1, 1` to `1, 1, 0` is `==` to an edge going from `1, 1, 0` to
/// `0, 1, 1`.
///
/// # Example
///
/// ```
/// use picocadrs::assets::{Edge, Point3D};
/// use picocadrs::point;
///
/// let e = Edge::new(point!(0.0, 1.0, 1.0), point!(1.0, 1.0, 0.0));
/// let f = Edge::new(point!(1.0, 1.0, 0.0), point!(0.0, 1.0, 1.0));
///
/// assert_eq!(e, f);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub start: Point3D<f64>,
    pub end: Point3D<f64>,
}

impl Edge {
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

#[cfg(test)]
pub mod tests {
    use crate::point;
    use super::*;

    #[test]
    fn test_eq() {
        let e = Edge::new(point!(0.0, 1.0, 1.0), point!(1.0, 1.0, 0.0));
        let f = Edge::new(point!(1.0, 1.0, 0.0), point!(0.0, 1.0, 1.0));

        assert_eq!(e, f);
    }
}