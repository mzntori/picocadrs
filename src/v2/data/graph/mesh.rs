//! This module holds any data for meshes.

use serde::{Deserialize, Serialize};

use crate::v2::data::graph::face::Face;

/// Represents a singular mesh.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Mesh {
    /// Represents the name of the mesh.
    /// This is not the name that is shown in the project overview or motion mode.
    /// I'm honestly not really sure what this is used for.
    name: String,
    /// This vector holds the positions of the verices of this mesh.
    /// While it is a 1-dimensional vector each pair of 3 values represents x, y and z coordinates relative to the transform applied to this mesh.
    /// So if the first 3 items of this vector are `1`, `-1`, `-1` vertices index `0` represents the coordinates `x=1, y=-1, z=-1`.
    vertices: Vec<f64>,
    /// Represents the faces of this mesh.
    faces: Vec<Face>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let foo: Mesh = serde_json::from_str(EXAMPLE).unwrap();
        let s = serde_json::to_string(&foo).unwrap();

        assert_eq!(foo, serde_json::from_str(&s).unwrap());
    }

    const EXAMPLE: &str = r#"
    {
      "name":"pyramid",
      "vertices":[-0.5,-0.5,-0.5,0.5,-0.5,-0.5,0.5,-0.5,0.5,-0.5,-0.5,0.5,0,0.5,0],
      "faces":[{
          "color":7,
          "uvs":[0.25,0,0.25,0.125,0.125,0.125,0.125,0],
          "vertex_ids":[1,2,3,4,5]
        },{
          "color":7,
          "uvs":[0.1875,-0.015625,0.25,0.125,0.125,0.125],
          "vertex_ids":[5,2,1]
        },{
          "color":7,
          "uvs":[0.1875,-0.015625,0.25,0.125,0.125,0.125],
          "vertex_ids":[5,3,2]
        },{
          "color":7,
          "uvs":[0.1875,-0.015625,0.25,0.125,0.125,0.125],
          "vertex_ids":[5,4,3]
        },{
          "color":7,
          "uvs":[0.1875,-0.015625,0.25,0.125,0.125,0.125],
          "vertex_ids":[5,1,4]
        }]
    }"#;
}
