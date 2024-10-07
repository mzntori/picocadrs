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
//! More information on how float coordinates work can be found in the docs of [`Footer`](super::Footer).
//!
//! *: picoCAD doesn't actually check the value of these fields but only if they exist.

use crate::assets::edge::Edge;
#[cfg(feature = "svg")]
use crate::assets::SVGAngle;
use crate::assets::{Color, Point2D, Point3D};
use crate::error::PicoError;
use crate::point;
use rlua::{Lua, Table, Value};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
#[cfg(feature = "svg")]
use svg::node::element::path::Data;

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
    /// use picocadrs::assets::{UVMap, Point2D};
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

impl Face {
    /// Generates a vector of [`edges`](Edge) that this face is formed by.
    /// Returns an empty vector if no edges can be generated or some edges fail.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::point;
    /// use picocadrs::assets::{Point3D, Face};
    ///
    /// let face = "{4,3,2,1, c=10, dbl=1, noshade=1, notex=1, prio=1, \
    ///     uv={16.25,0,1.25,0,15.5,2,-0.75,2} }"
    ///     .parse::<Face>()
    ///     .unwrap();
    ///
    /// dbg!(face.edges(&vec![
    ///     point!(0.0, 1.0, 0.0),
    ///     point!(0.0, 0.0, 0.0),
    ///     point!(1.0, 0.0, 0.0),
    ///     point!(1.0, 1.0, 0.0),
    /// ]));
    /// ```
    pub fn edges(&self, mesh_vertices: &[Point3D<f64>]) -> Vec<Edge> {
        if self.uv_maps.len() < 2 {
            return vec![];
        }

        let start = match mesh_vertices.get(self.uv_maps[0].vertex_index) {
            Some(point) => point,
            None => {
                return vec![];
            }
        };

        let mut result: Vec<Edge> = vec![];
        let mut last: &Point3D<f64> = start;

        for uv_map in self.uv_maps.iter() {
            let vertex = match mesh_vertices.get(uv_map.vertex_index) {
                None => continue,
                Some(v) => v,
            };

            if vertex != last {
                result.push(Edge::new(*last, *vertex))
            }

            last = vertex
        }

        result.push(Edge::new(*last, *start));

        result
    }

    /// Returns a vector of all vertices a face touches in order.
    pub fn vertices(&self, mesh_vertices: &[Point3D<f64>]) -> Vec<Point3D<f64>> {
        let mut vertices: Vec<Option<&Point3D<f64>>> = vec![];

        for uv_map in self.uv_maps.iter() {
            vertices.push(mesh_vertices.get(uv_map.vertex_index));
        }

        vertices.into_iter().flatten().copied().collect()
    }

    /// Generates SVG path data for all edges of this face.
    /// Requires the `svg` feature.
    ///
    /// For more information on how to use the path data, take a look at the [`svg`](https://docs.rs/svg/latest/svg/index.html) crate.
    #[cfg(feature = "svg")]
    pub fn svg_path_data(
        &self,
        mesh_vertices: &[Point3D<f64>],
        angle: SVGAngle,
        scale: f64,
        offset: Point2D<f64>,
    ) -> Data {
        let mut data = Data::new();

        for (i, vertex) in self.vertices(mesh_vertices).iter().enumerate() {
            data = if i == 0 {
                data.move_to(vertex.svg_position(angle, scale, offset))
            } else {
                data.line_to(vertex.svg_position(angle, scale, offset))
            }
        }

        data.close()
    }

    /// Generates a string of SVG path data for all edges of this face.
    /// Requires the `svg` feature.
    ///
    /// Will Always generate a closed path.
    #[cfg(feature = "svg")]
    pub fn svg_path(
        &self,
        mesh_vertices: &[Point3D<f64>],
        angle: SVGAngle,
        scale: f64,
        offset: Point2D<f64>,
    ) -> String {
        let mut path = String::new();

        for (i, vertex) in self.vertices(mesh_vertices).iter().enumerate() {
            let pos = vertex.svg_position(angle, scale, offset);

            if i == 0 {
                path.push_str(format!("M{},{} ", pos.0, pos.1).as_str());
            } else {
                path.push_str(format!("L{},{} ", pos.0, pos.1).as_str());
            }
        }

        path.push('z');
        path
    }
}

impl Default for Face {
    /// Creates a new face that is attached to no vertices.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{Face, Color};
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
            uvs.push_str(format!("{},", uv_map.coords).as_str());
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
    type Error = PicoError;

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
                            return Err(PicoError::FaceUVMapLength(uv_maps.len(), uv_chunks.len()));
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
    type Err = PicoError;

    /// Parses a face from a string that contains a lua table with the right arguments.
    ///
    /// # Exmaple
    ///
    /// ```
    /// use picocadrs::assets::{Face, UVMap, Color, Point2D};
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
                Err(err) => Err(PicoError::from(err)),
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

    #[test]
    fn test_edges() {
        let face = "{4,3,2,1, c=10, dbl=1, noshade=1, notex=1, prio=1, \
        uv={16.25,0,1.25,0,15.5,2,-0.75,2} }"
            .parse::<Face>()
            .unwrap();

        dbg!(face.edges(&[
            point!(0.0, 1.0, 0.0),
            point!(0.0, 0.0, 0.0),
            point!(1.0, 0.0, 0.0),
            point!(1.0, 1.0, 0.0),
        ]));
    }
}

#[cfg(test)]
#[cfg(feature = "svg")]
pub mod tests_svg {
    use super::*;
    use crate::assets::Mesh;
    #[cfg(feature = "svg")]
    use svg::{node::element::Path, Document};

    #[test]
    #[cfg(feature = "svg")]
    fn test_face_svg() {
        let mesh = TEST_MESH.parse::<Mesh>().unwrap();

        let mut document = Document::new().set("viewBox", (-100, -100, 200, 200));

        for face in mesh.faces.clone() {
            document = document.add(
                Path::new()
                    .set("fill", "none")
                    .set("stroke", format!("#{}", face.color.as_hex()))
                    .set("stroke-width", 1)
                    .set(
                        "d",
                        face.svg_path_data(&mesh.vertices, SVGAngle::X, 5.0, point!(0.0, 0.0)),
                    ),
            );
        }

        svg::save("test_output_files/svg_face_test_x.svg", &document).unwrap();
    }

    #[test]
    #[cfg(feature = "svg")]
    fn test_face_svg_path() {
        let mesh = TEST_MESH.parse::<Mesh>().unwrap();

        for face in mesh.faces.iter() {
            dbg!(face.svg_path(&mesh.vertices, SVGAngle::X, 5.0, point!(0.0, 0.0)));
        }
    }

    const TEST_MESH: &str = r#"{
 name='branch', pos={0,-30,0}, rot={0,0,0},
 v={
  {-6.9584,14.25,3},
  {-5.1951,12.75,8.25},
  {-7.4383,13.5,3},
  {-6.9576,13.5,2.75},
  {-5.3511,11.5,7.75},
  {-6.3128,11.75,7.75},
  {-7.438,13.75,2.5},
  {-6.9573,13.75,2},
  {-6.9581,14.5,2.5},
  {-6.3173,15.75,-2},
  {-5.8374,15.75,-2},
  {-5.8377,16,-1.5},
  {-6.3172,16,-1.75},
  {-5.9998,15.5,-2.25},
  {-5.8374,15.75,-2},
  {-5.8377,16,-1.5},
  {-4.8806,16.25,-2.75},
  {-3.7565,16.75,-3.75},
  {-3.7565,17,-3.75},
  {-4.8806,16.5,-2.75},
  {-3.9176,16.25,1.25},
  {-5.6761,15,1.75},
  {-5.6756,15.5,2},
  {-3.9176,16.5,1.25},
  {-8.7127,15,-3.75},
  {-7.4386,15.5,-2.5},
  {-7.438,15.25,-2.75},
  {-8.7135,14.75,-3.75},
  {-5.5185,13.75,0},
  {-5.0384,17,-0.25},
  {-4.7158,14.25,4.25},
  {-5.0348,16.5,2.5},
  {-4.8806,15,-4.25},
  {-5.5195,17.75,-4.75},
  {-3.7576,14.25,-1},
  {-5.0405,18.25,-2.25},
  {-9.3571,14.25,-2.5},
  {-7.5993,15.25,-4.75},
  {-7.1164,16.25,-2},
  {-6.6399,17,-4.25},
  {-7.7545,13,-1.5},
  {-5.9975,15.75,-1.75},
  {-5.1952,12.75,2.75},
  {-4.0753,15.5,3.25}
 },
 f={
  {4,3,6,5, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {1,4,5,2, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {7,3,4,8, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {10,7,8,11, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {11,8,9,12, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {14,10,11,15, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {15,11,12,16, c=4, dbl=1, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {17,14,15,18, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {18,15,16,19, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {17,18,19,20, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {21,8,4,22, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {22,4,1,23, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {23,1,9,24, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {24,9,8,21, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {21,22,23,24, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {26,13,10,27, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {27,10,14,28, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {25,26,27,28, c=4, dbl=1, uv={13,4.5,14,4.5,14,5,13,5} },
  {31,32,30,29, c=6, dbl=1, noshade=1, uv={11,4.5,12.75,4.5,12.75,6,11.5,6} },
  {35,36,34,33, c=6, dbl=1, noshade=1, uv={10.75,4.5,12.75,4.5,12.25,6,10.75,6} },
  {39,40,38,37, c=6, dbl=1, noshade=1, uv={10.75,4.5,12.75,4.5,12.25,6,10.75,6} },
  {43,44,42,41, c=6, dbl=1, noshade=1, uv={10.5,4.5,12.5,4.5,12,6,10.5,6} }
 }
}"#;
}
