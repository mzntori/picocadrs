//! This Module holds any data connected to mesh faces.

use serde::{Deserialize, Serialize};

/// Represents a face of a mesh.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Face {
    /// This vector holds the uv mappings.
    /// While they are stored in a single dimension array they build pairs where index `0` and `1` represent the u and v coordinates  of the first vertex stored in [`vertex_ids`](Self::vertex_ids) and so on.
    /// This means it should always count: `uvs.len() == vertex_ids.len() * 2` otherwise the program can crash.
    uvs: Vec<f64>,
    /// Vector of indices for the vertices vector of this mesh representing the vertices of this face.
    vertex_ids: Vec<usize>,
    /// Represents the colorpalette index for the static color of this face.
    color: usize,
    /// If set to true the face will be rendered before any other, so it ends up in the background.
    /// If omitted picoCAD defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    prio: Option<bool>,
    /// If set to true the face will be rendered from both sides.
    /// If omitted picoCAD defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    dbl: Option<bool>,
    /// If set to true no texture will be rendered on this face and [`color`](Self::color) will be used instead.
    /// If omitted picoCAD defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    notex: Option<bool>,
    /// If set to true no shading will be applied to the face.
    /// If omitted picoCAD defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    noshade: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let foo: Face = serde_json::from_str(EXAMPLE).unwrap();
        let s = serde_json::to_string(&foo).unwrap();

        assert_eq!(foo, serde_json::from_str(&s).unwrap());
    }

    const EXAMPLE: &str = r#"
    	{
    		"uvs":[0.25,0,0.25,0.125,0.125,0.125,0.125,0],
    		"vertex_ids":[1,2,3,4],
    		"color":7
    	}
    "#;
}
