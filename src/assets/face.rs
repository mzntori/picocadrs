use std::fmt::{Display, Formatter};
use crate::assets::color::Color;
use crate::assets::point::Point2D;

/// Represents uv-coordinates and the vertex they correspond to.
///
/// When building a face this helps with keeping index corresponding uv-coordinates together.
/// Since internally these two are stored separately from each other, I think this also makes it
/// easier to understand.
///
/// uv-mappings are stored inside the face of a mesh.
///
/// <br/>
///
/// Indexes are not the same they are in the project files.
/// picoCAD uses indexes starting from 1 for referencing vertices.
/// To make it more in line with standard programming rules they start from 0 here and only will be
/// converted into the actual indexes when serializing.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UVMap {
    pub vertex_index: usize,
    pub coords: Point2D<f64>,
}

impl UVMap {
    /// Creates a new `UVMap`.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::face::UVMap;
    /// use picocadrs::uv;
    ///
    /// let map = UVMap::new(2, uv!(2.0, 3.5));
    ///
    /// assert_eq!(map.vertex_index, 2);
    /// assert_eq!(map.coords.u, 2.0);
    /// assert_eq!(map.coords.v, 3.5);
    /// ```
    pub fn new(vertex_index: usize, coords: Point2D<f64>) -> UVMap {
        UVMap {
            vertex_index,
            coords,
        }
    }
}

/// Represents the face of a mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    /// If true, face will get rendered from both sides.
    pub double_sided: bool,
    /// If true, no shading will be applied to this face.
    pub no_shading: bool,
    /// If true, this face will be rendered first.
    /// This means it will appear behind all other faces.
    pub render_priority: bool,
    /// If true, no texture will be rendered on this face.
    pub no_texture: bool,
    /// Color of the face. If `no_texture` is enabled this color will show.
    pub color: Color,
    /// uv-mappings of this face.
    /// Tells picoCAD which vertices this face is between and where they are on the uv-map.
    pub uv_maps: Vec<UVMap>,
}

impl Default for Face {
    /// Creates a new face that is attached to no vertices.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{face::Face, color::Color};
    ///
    /// let face = Face::default();
    ///
    /// assert!(!face.double_sided);
    /// assert!(!face.no_shading);
    /// assert!(!face.render_priority);
    /// assert!(!face.no_texture);
    /// assert!(face.uv_maps.is_empty());
    /// assert_eq!(face.color, Color::Black);
    /// ```
    fn default() -> Self {
        Face {
            double_sided: false,
            no_shading: false,
            render_priority: false,
            no_texture: false,
            color: Color::Black,
            uv_maps: vec![],
        }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vertex_indices = String::new();
        let mut uvs = String::new();

        for uv_map in self.uv_maps.iter() {
            vertex_indices.push_str(format!("{},", uv_map.vertex_index + 1).as_str());
            uvs.push_str(format!("{},", uv_map.coords.to_string()).as_str());
        }

        let mut attributes = String::new();

        if self.double_sided {
            attributes.push_str("dbl=1, ");
        }

        if self.no_shading {
            attributes.push_str("noshade=1, ");
        }

        if self.no_texture {
            attributes.push_str("notex=1, ");
        }

        if self.render_priority {
            attributes.push_str("prio=1, ");
        }
        // formats:   v     v    v       v
        write!(f, "{{{ } c={ }, { }uv={{{ }}} }}",
               vertex_indices,
               self.color.as_i32(),
               attributes,
               uvs.trim_end_matches(',')
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::uv;
    use super::*;

    #[test]
    fn test_uvmap_new() {
        let map = UVMap::new(2, uv!(2.0, 3.5));

        assert_eq!(map.vertex_index, 2);
        assert_eq!(map.coords.u, 2.0);
        assert_eq!(map.coords.v, 3.5);
    }

    #[test]
    fn test_face_default() {
        let face = Face::default();

        assert!(!face.double_sided);
        assert!(!face.no_shading);
        assert!(!face.render_priority);
        assert!(!face.no_texture);
        assert!(face.uv_maps.is_empty());
        assert_eq!(face.color, Color::Black);
    }

    #[test]
    fn test_face_display() {
        let mut face = Face::default();

        face.uv_maps.push(UVMap::new(0, uv!(2.0, 3.5)));
        face.uv_maps.push(UVMap::new(2, uv!(1.0, 3.5)));
        face.uv_maps.push(UVMap::new(1, uv!(1.5, 2.0)));
        face.no_texture = true;

        assert_eq!(
            face.to_string(),
            "{1,3,2, c=0, notex=1, uv={2,3.5,1,3.5,1.5,2} }"
        )
    }
}