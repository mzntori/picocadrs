//! Module housing logic for faces.
//! Faces are planes spanning between multiple vertices textures can be mapped onto.
//!
//! In a picoCAD File a face could look something like this:
//!
//! `f={{3,2,1, c=10, dbl=1, noshade=1, notex=1, prio=1, uv={1.25,0,15.5,2,-0.75,2} }} `
//!
//! Which lies within a mesh.
//!
//! - _c (color):_ The color the mesh has if no texture is mapped onto it.
//! Color index is given in decimal. For more information look at the [`color`](super::color) module or
//! [this table](https://pico-8.fandom.com/wiki/Palette#0..15:_Official_base_colors).
//! In this case `10` which represents yellow.
//!
//! - _dbl (double-sided):_ if existent* the face will be rendered from both sides.
//!
//! - _noshade (no shading):_ if existent* the face will not show any shadows on it.
//!
//! - _notex (no texture):_ if existent* the face will not have textures mapped onto it and will
//! just be the color of the _c_ field.
//!
//! - _prio (render priority):_ if existent* the face will be rendered before any other face
//! leading to it always being behind all other faces.
//!
//! - _table indices:_ In this case the values `3,2,1` at the start of the table.
//! Indicate which vertices of the mesh this face lives within to use as corners.
//! Indexing starts at 1, meaning this face uses the first three vertices of the mesh it is within.
//! Order also matters as it tells picoCAD in which orders to draw edges.
//! In the example of `3,2,1` it goes `3 -> 2 -> 1 -> 3`.
//! This means that `4,3,2,1` is not the same face as `3,4,2,1`.
//!
//! - _uv:_ Represents the coordinates on the texture that are mapped to corners of the face.
//! Always paired into 2 values.
//! This is in relation to the _table indices_ as their positions determine which vertex gets which
//! coordinates.
//! In the example above the coordinates `1.25, 0` in the texture are mapped onto the corner that
//! is at vertex with the index `3`.
//! More information on how float coordinates work can be found in the docs of [`Footer`](super::footer::Footer).
//!
//! *: picoCAD doesn't actually check the value of these fields but only if they exist.

use crate::assets::{color::Color, point::Point2D};
use crate::error::PicoParseError;
use crate::point;
use rlua::{Lua, Table, Value};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
    /// use picocadrs::point;
    ///
    /// let map = UVMap::new(2, point!(2.0, 3.5));
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

        write!(
            f,
            "{{{ } c={ }, { }uv={{{ }}} }}",
            vertex_indices,
            self.color.as_i32(),
            attributes,
            uvs.trim_end_matches(',')
        )
    }
}

impl TryFrom<Table<'_>> for Face {
    type Error = PicoParseError;

    /// Tries to create a [`Face`] from a lua table.
    ///
    /// If you have a lua-table in form of a string try parsing from that string.
    fn try_from(value: Table<'_>) -> Result<Self, Self::Error> {
        let mut color = Color::Invalid;
        let mut uv_maps: Vec<UVMap> = vec![];
        let mut double_sided: bool = false;
        let mut no_shading: bool = false;
        let mut no_texture: bool = false;
        let mut render_priority: bool = false;

        for seq_value in value.clone().sequence_values::<usize>() {
            uv_maps.push(UVMap::new(seq_value? - 1, point!(0.0, 0.0)));
        }

        for pair in value.pairs::<String, Value>() {
            let (key, value) = pair.unwrap();

            match key.as_str() {
                "dbl" => double_sided = true,
                "noshade" => no_shading = true,
                "notex" => no_texture = true,
                "prio" => render_priority = true,
                "c" => {
                    color = match value {
                        Value::Integer(int) => Color::from(int as i32),
                        _ => Color::Invalid,
                    }
                }
                "uv" => {
                    if let Value::Table(table) = value {
                        let uv_chunks = table
                            .sequence_values::<f64>()
                            .map_while(|v| v.ok())
                            .collect::<Vec<f64>>();

                        // return error if lengths don't match.
                        if uv_chunks.len() != uv_maps.len() * 2 {
                            return Err(PicoParseError::FaceUVMapLength(
                                uv_maps.len(),
                                uv_chunks.len(),
                            ));
                        }

                        for (i, chunk) in uv_chunks.chunks_exact(2).enumerate() {
                            uv_maps[i].coords = point!(chunk[0], chunk[1]);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Face {
            double_sided,
            no_texture,
            no_shading,
            render_priority,
            uv_maps,
            color,
        })
    }
}

impl FromStr for Face {
    type Err = PicoParseError;

    /// Parses a face from a string that contains a lua table with the right arguments.
    ///
    /// # Exmaple
    ///
    /// ```
    /// use picocadrs::assets::{face::{Face, UVMap}, color::Color, point::Point2D};
    /// use picocadrs::point;
    ///
    /// assert_eq!(
    ///     "{1,3,2, c=0, notex=1, uv={2,3.5,1,3.5,1.5,2} }",
    ///     "{1,3,2, c=0, notex=1, uv={2,3.5,1,3.5,1.5,2} }".parse::<Face>().unwrap().to_string()
    /// );
    ///
    /// let face = "{4,3,2,1, c=10, dbl=1, noshade=1, notex=1, prio=1, uv={16.25,0,1.25,0,15.5,2,-0.75,2} }".parse::<Face>().unwrap();
    ///
    /// assert_eq!(face.color, Color::from(10));
    /// assert!(face.double_sided);
    /// assert!(face.no_shading);
    /// assert!(face.no_texture);
    /// assert!(face.render_priority);
    /// assert_eq!(face.uv_maps[1], UVMap::new(2, point!(1.25, 0.0)));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut face = Ok(Face::default());

        let lua = Lua::new();
        lua.context(|ctx| {
            let table_result: rlua::Result<Table> = ctx.load(s).eval();

            face = match table_result {
                Ok(table) => Face::try_from(table),
                Err(err) => Err(PicoParseError::from(err)),
            }
        });

        face
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::point;

    #[test]
    fn test_uvmap_new() {
        let map = UVMap::new(2, point!(2.0, 3.5));

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

        face.uv_maps.push(UVMap::new(0, point!(2.0, 3.5)));
        face.uv_maps.push(UVMap::new(2, point!(1.0, 3.5)));
        face.uv_maps.push(UVMap::new(1, point!(1.5, 2.0)));
        face.no_texture = true;

        assert_eq!(
            face.to_string(),
            "{1,3,2, c=0, notex=1, uv={2,3.5,1,3.5,1.5,2} }"
        )
    }

    #[test]
    fn test_face_parse() {
        assert_eq!(
            "{1,3,2, c=0, notex=1, uv={2,3.5,1,3.5,1.5,2} }",
            "{1,3,2, c=0, notex=1, uv={2,3.5,1,3.5,1.5,2} }"
                .parse::<Face>()
                .unwrap()
                .to_string()
        );

        let face = "{4,3,2,1, c=10, dbl=1, noshade=1, notex=1, prio=1, \
        uv={16.25,0,1.25,0,15.5,2,-0.75,2} }"
            .parse::<Face>()
            .unwrap();

        assert_eq!(face.color, Color::from(10));
        assert!(face.double_sided);
        assert!(face.no_shading);
        assert!(face.no_texture);
        assert!(face.render_priority);
        assert_eq!(face.uv_maps[1], UVMap::new(2, point!(1.25, 0.0)));
    }
}
