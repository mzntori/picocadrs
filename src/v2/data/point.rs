use serde::{Deserialize, Serialize};

/// Represents a point in 3-dimensional space.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}
