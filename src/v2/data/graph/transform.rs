//! This module holds data for transformation.
//! Transformation is applied to nodes.

use serde::{Deserialize, Serialize};

use crate::v2::data::point::Point;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Transform {
    /// Represents a translation in 3-dimensional space.
    pub pos: Point,
    /// Represents a rotation in 3-dimensional space.
    /// A rotation of pi is equal to half a rotaion.
    /// Rotation is applied in the order of `x` -> `y` -> `z`.
    pub rot: Point,
    /// Represents axis based scaling.
    pub scale: Point,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let foo: Transform = serde_json::from_str(EXAMPLE).unwrap();
        let s = serde_json::to_string(&foo).unwrap();

        assert_eq!(foo, serde_json::from_str(&s).unwrap());
    }

    const EXAMPLE: &str = r#"
    {
        "pos":{
          "x":0,
          "y":0,
          "z":0
        },
        "rot":{
          "x":0,
          "y":0,
          "z":0
        },
        "scale":{
          "x":2,
          "y":1,
          "z":1
        }
    }"#;
}
