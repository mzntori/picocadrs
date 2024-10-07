//! In charge of mesh level data.
//!
//! A mesh has 5 fields.
//! - _name:_ Name of the mesh.
//! To reduce file-space this can be shortened to a singular character which will not affect the
//! render result.
//! - _pos (position):_ Anchor point of the mesh given as a point in 3-dimensional space.
//! All Vertex positions are relative to this position.
//! - _rot (rotation):_ Shadow rotation of the mesh.
//! More info in [`Rotation`].
//! - _v (vertices):_ List of all the vertices relative to _position_ given as a point in
//! 3-dimensional space.
//! - _f (faces):_ List of all faces the mesh has.
//! More info on faces [`here`](crate::assets::face).
//!
//! This module also provides a wrapper struct for [`rotation`](Rotation) which implements some useful methods
//! that only apply to rotation in picoCAD.

use crate::assets::edge::Edge;

use crate::{
    assets::{Face, Point3D},
    error::PicoError,
    point,
};

#[cfg(feature = "svg")]
use crate::assets::Point2D;
#[cfg(feature = "svg")]
use crate::assets::SVGAngle;
use rlua::{Lua, Table, Value};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
#[cfg(feature = "svg")]
use svg::node::element::path::Data;

/// Wrapper type for [`Point3D<f64>`] representing a rotation in picoCAD.
/// If you want to access the raw [`Point3D`] type that is wrapped you can access it using an index
/// onto its first element using `.0`.
///
/// <br/>
///
/// Rotations in picoCAD can go from `0.0` to `1.0` meaning a value of `1.0` represents 360-degree
/// rotation on a given axis.
///
/// Rotation in picoCAD also doesn't mean rotation of the mesh itself but from what angle the
/// "light source" shines on it relative to the camera angle.
/// This means that setting the rotation of a mesh to `0.5` on one axis will make it so the mesh
/// will be shined on from the opposite side of the camera.
///
/// # Example
///
/// ```
/// use picocadrs::assets::{Point3D, Rotation};
/// use picocadrs::point;
///
/// let rot = Rotation(point!(0.3, 0.2, 0.1));
/// assert_eq!(rot.0, point!(0.3, 0.2, 0.1));
/// ```
///
/// # Important Notes
///
/// To check if the actual rotation of two instances is equal (rounded to the 3rd
/// digit), normalizing and rounding fill not be enough in some rare cases.
///
/// ```
/// use picocadrs::assets::{Rotation, Point3D};
/// use picocadrs::point;
///
/// let mut rot = Rotation(point!(0.9999, 1.0, 0.0));
/// rot.normalize();
/// rot.round();
///
/// // This should be equal to 0.0, 0.0, 0.0
/// assert_eq!(rot, Rotation(point!(1.0, 0.0, 0.0)));
/// ```
///
/// As you can see in the case above, `0.9999` gets rounded to `1.0` after it was normalized
/// but `1.0` in the `y` field gets normalized to `0.0`.
/// This is what [`equal_rotation`](Rotation::equal_rotation) does:
///
/// ```
/// use picocadrs::assets::{Rotation, Point3D};
/// use picocadrs::point;
///
/// let mut rot = Rotation(point!(0.9999, 1.0, 0.0));
/// rot.round();
/// rot.normalize();
/// rot.round();
///
/// assert_eq!(rot, Rotation(point!(0.0, 0.0, 0.0)));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rotation(pub Point3D<f64>);

impl Rotation {
    /// Rounds each component to 3 digits behind the comma.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{Rotation, Point3D};
    /// use picocadrs::point;
    ///
    /// let mut rot = Rotation(point!(0.2423, 0.9999, 0.34));
    /// rot.round();
    ///
    /// assert_eq!(rot, Rotation(point!(0.242, 1.0, 0.34)));
    /// ```
    pub fn round(&mut self) {
        self.0.map(|v| (v * 1000.0).round() / 1000.0)
    }

    /// Normalizes the values in the rotation, so it takes the least possible amount of storage.
    ///
    /// That means it takes every value modulo 1:
    ///
    /// `12.24 % 1 = 0.24` (also `1.0 % 1 - 0.0`!)
    ///
    /// and gets rid of any minus by converting the rotation into its positive counterpart as
    /// `-0.21 = 0.79` in picoCAD rotation.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{Rotation, Point3D};
    /// use picocadrs::point;
    ///
    /// let mut rot = Rotation(point!(2.24, -1.21, 0.34));
    /// rot.normalize();
    /// rot.round();
    ///
    /// assert_eq!(rot, Rotation(point!(0.24, 0.79, 0.34)));
    /// ```
    pub fn normalize(&mut self) {
        self.0.map(|v| {
            let mut a = v % 1.0;
            a += 1.0;
            a % 1.0
        });
    }

    /// Checks if `self` represents the same rotation as `other` up to the third digit after the
    /// comma.
    /// This operation is _not_ the same as normal [`==`](PartialEq) comparisons on this type.
    /// The [`PartialEq`](PartialEq) implementation on this struct does not check if the _rotation_
    /// is the same, but if the individual values are the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use picocadrs::assets::{Rotation, Point3D};
    /// use picocadrs::point;
    ///
    /// let rot = Rotation(point!(2.0, 1.5, 0.0));
    /// let actual_rot = Rotation(point!(0.0, 0.5, 0.0));
    ///
    /// assert_ne!(rot, actual_rot);
    /// assert!(rot.equal_rotation(&actual_rot));
    /// ```
    pub fn equal_rotation(&self, other: &Rotation) -> bool {
        let mut left = *self;
        let mut right = *other;

        left.round();
        left.normalize();
        left.round();

        right.round();
        right.normalize();
        right.round();

        left == right
    }
}

/// Represents a mesh inside a picoCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    /// Name of the mesh.
    /// To safe file space this can be set to singular characters.
    pub name: String,
    /// Position of the mesh.
    /// Acts as an anchor point.
    pub position: Point3D<f64>,
    /// Shadow rotation on the mesh.
    pub rotation: Rotation,
    /// Vertices the mesh consists of.
    /// Coordinates are relative to `position` field.
    pub vertices: Vec<Point3D<f64>>,
    /// Faces of a mesh.
    pub faces: Vec<Face>,
}

impl Mesh {
    /// Creates a new mesh with the given name.
    /// Position and rotation will be set to `0.0, 0.0, 0.0`.
    /// Vertex and face vectors are empty.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{Mesh, Point3D};
    /// use picocadrs::point;
    ///
    /// let mesh = Mesh::new("my_mesh".to_string());
    ///
    /// assert_eq!(mesh.name, "my_mesh");
    /// assert_eq!(mesh.position, point!(0.0, 0.0,0.0));
    /// assert_eq!(mesh.rotation.0, point!(0.0, 0.0,0.0));
    /// assert!(mesh.faces.is_empty());
    /// assert!(mesh.vertices.is_empty());
    /// ```
    pub fn new(name: String) -> Mesh {
        Mesh {
            name,
            position: point!(0.0, 0.0, 0.0),
            rotation: Rotation(point!(0.0, 0.0, 0.0)),
            vertices: vec![],
            faces: vec![],
        }
    }

    /// Generates a vector containing all edges this mesh owns.
    pub fn edges(&self) -> Vec<Edge> {
        let mut face_edges: Vec<Edge> = vec![];

        for face in self.faces.iter() {
            let edges = face.edges(&self.vertices);

            for edge in edges {
                if !face_edges.contains(&edge) {
                    face_edges.push(edge)
                }
            }
        }

        face_edges
    }

    /// Returns a vector of SVG path data for each face of the mesh.
    /// Requires the `svg` feature.
    ///
    /// For more information on how to use the path data, take a look at the [`svg`](https://docs.rs/svg/latest/svg/index.html) crate.
    #[cfg(feature = "svg")]
    pub fn svg_path_data(&self, angle: SVGAngle, scale: f64, offset: Point2D<f64>) -> Vec<Data> {
        let mut data: Vec<Data> = vec![];

        for face in self.faces.iter() {
            data.push(face.svg_path_data(&self.vertices, angle, scale, offset));
        }

        data
    }

    #[cfg(feature = "svg")]
    pub fn svg_path(&self, angle: SVGAngle, scale: f64, offset: Point2D<f64>) -> String {
        let mut path = String::new();



        path
    }
}

impl Display for Mesh {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name: String = self.name.clone();
        let pos: String = format!("{{{}}}", self.position);
        let rot: String = format!("{{{}}}", self.rotation.0);

        let mut v: String = String::new();

        for (i, vertex) in self.vertices.iter().enumerate() {
            v.push_str(format!("  {{{}}}", vertex).as_str());
            if i + 1 < self.vertices.len() {
                v.push_str(",\n");
            }
        }

        let mut f: String = String::new();

        for (i, face) in self.faces.iter().enumerate() {
            f.push_str(format!("  {}", face).as_str());
            if i + 1 < self.faces.len() {
                f.push_str(",\n");
            }
        }

        write!(
            formatter,
            "{{\n name='{}', pos={}, rot={},\n v={{\n{}\n }},\n f={{\n{}\n }}\n}}",
            name, pos, rot, v, f
        )
    }
}

impl TryFrom<Table<'_>> for Mesh {
    type Error = PicoError;

    fn try_from(value: Table<'_>) -> Result<Self, Self::Error> {
        let mut name = String::new();
        let mut position: Point3D<f64> = point!(0.0, 0.0, 0.0);
        let mut rotation = Rotation(point!(0.0, 0.0, 0.0));
        let mut vertices: Vec<Point3D<f64>> = vec![];
        let mut faces: Vec<Face> = vec![];

        for pair in value.pairs::<String, Value>() {
            let (key, value) = pair.unwrap();

            match key.as_str() {
                "name" => {
                    name = if let Value::String(string) = value {
                        string.to_str()?.to_string()
                    } else {
                        return Err(PicoError::MeshField("name".to_string()));
                    }
                }
                "pos" => {
                    position = if let Value::Table(table) = value {
                        Point3D::try_from(table)?
                    } else {
                        return Err(PicoError::MeshField("pos".to_string()));
                    }
                }
                "rot" => {
                    rotation = if let Value::Table(table) = value {
                        Rotation(Point3D::try_from(table)?)
                    } else {
                        return Err(PicoError::MeshField("rot".to_string()));
                    }
                }
                "v" => {
                    if let Value::Table(table) = value {
                        for point in table.sequence_values::<Table>() {
                            vertices.push(Point3D::try_from(point?)?);
                        }
                    } else {
                        return Err(PicoError::MeshField("rot".to_string()));
                    };
                }
                "f" => {
                    if let Value::Table(table) = value {
                        for face in table.sequence_values::<Table>() {
                            faces.push(Face::try_from(face?)?);
                        }
                    } else {
                        return Err(PicoError::MeshField("rot".to_string()));
                    }
                }
                _ => {}
            }
        }

        Ok(Mesh {
            name,
            position,
            rotation,
            vertices,
            faces,
        })
    }
}

impl FromStr for Mesh {
    type Err = PicoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mesh = Ok(Mesh::new("mesh".to_string()));

        let lua = Lua::new();
        lua.context(|ctx| {
            let table_result: rlua::Result<Table> = ctx.load(s).eval();

            mesh = match table_result {
                Ok(table) => Mesh::try_from(table),
                Err(err) => Err(PicoError::from(err)),
            }
        });

        mesh
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::point;

    #[test]
    fn test_rot_round() {
        let mut rot = Rotation(point!(0.2423, 0.9999, 0.34));
        rot.round();

        assert_eq!(rot, Rotation(point!(0.242, 1.0, 0.34)));
    }

    #[test]
    fn test_rot_normalize() {
        let mut rot = Rotation(point!(2.24, -1.21, 0.34));
        rot.normalize();
        rot.round();

        assert_eq!(rot, Rotation(point!(0.24, 0.79, 0.34)));
    }

    #[test]
    fn test_rot_equal_rotation() {
        let mut rot = Rotation(point!(0.9999, 1.0, 0.0));
        rot.normalize();
        rot.round();

        assert_eq!(rot, Rotation(point!(1.0, 0.0, 0.0)));

        let mut rot = Rotation(point!(0.9999, 1.0, 0.0));
        rot.round();
        rot.normalize();
        rot.round();

        assert_eq!(rot, Rotation(point!(0.0, 0.0, 0.0)));

        assert!(rot.equal_rotation(&Rotation(point!(0.0, 0.0, 0.0))));
    }

    #[test]
    fn test_mesh_new() {
        let mesh = Mesh::new("my_mesh".to_string());

        assert_eq!(mesh.name, "my_mesh");
        assert_eq!(mesh.position, point!(0.0, 0.0, 0.0));
        assert_eq!(mesh.rotation.0, point!(0.0, 0.0, 0.0));
        assert!(mesh.faces.is_empty());
        assert!(mesh.vertices.is_empty());
    }

    #[test]
    fn test_mesh_parse() {
        assert_eq!(TEST_MESH, TEST_MESH.parse::<Mesh>().unwrap().to_string());
    }

    #[test]
    fn test_mesh_edges() {
        let mesh = TEST_MESH.parse::<Mesh>().unwrap();

        dbg!(mesh.edges());
    }

    const TEST_MESH: &str = r#"{
 name='cube', pos={0,0,0}, rot={0,-0.5,0},
 v={
  {-0.5,-0.5,-0.5},
  {0.5,-0.5,-0.5},
  {0.5,0.5,-0.5},
  {-0.5,0.5,-0.5},
  {-0.5,-0.5,0.5},
  {0.5,-0.5,0.5},
  {0.5,0.5,0.5},
  {-0.5,0.5,0.5}
 },
 f={
  {1,2,3,4, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {6,5,8,7, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {5,6,2,1, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {5,1,4,8, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {2,6,7,3, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {4,3,7,8, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} }
 }
}"#;
}

#[cfg(test)]
#[cfg(feature = "svg")]
pub mod tests_svg {
    use super::*;
    #[cfg(feature = "svg")]
    use svg::{node::element::Path, Document};

    #[test]
    #[cfg(feature = "svg")]
    fn test_svg() {
        let mesh = TEST_MESH.parse::<Mesh>().unwrap();
        let datas = mesh.svg_path_data(SVGAngle::Z, 20.0, point!(0.0, 0.0));

        let mut document = Document::new()
            .set("viewBox", (-100, -100, 200, 200));

        for (data, face) in datas.into_iter().zip(mesh.faces) {
            let path = Path::new()
                .set("fill", format!("#{}55", face.color.as_hex()))
                .set("stroke", format!("#{}", face.color.as_hex()))
                .set("stroke-width", 1)
                .set("d", data);

            document = document.add(path);
        }


        svg::save("test_output_files/svg_test_x.svg", &document).unwrap();
    }

    const TEST_MESH: &str = r#"{
 name='foxBody', pos={0,0,0}, rot={0,0,0},
 v={
  {-3.3605,-2.3801,-1.6364},
  {-3.8399,-2.6998,-1.3173},
  {-3.8395,-2.6998,-0.6753},
  {-3.3597,-2.3801,-0.3585},
  {-3.68,-1.7399,-0.36},
  {-3.6797,-1.1001,-0.6789},
  {-3.6798,-1.1001,-1.3183},
  {-3.6803,-1.7399,-1.6371},
  {-4.3195,-2.2201,-1.4824},
  {-4.3199,-2.3798,-1.3187},
  {-4.3197,-2.3798,-0.6781},
  {-4.3189,-2.2201,-0.511},
  {-4.3189,-1.8998,-0.511},
  {-4.3198,-1.2601,-0.8409},
  {-4.3199,-1.2601,-1.1591},
  {-4.3195,-1.8998,-1.4824},
  {-2.4006,-2.2199,-1.6376},
  {-2.08,-2.5395,-1.3183},
  {-2.0794,-2.5395,-0.6769},
  {-2.3993,-2.2199,-0.3578},
  {-2.7187,-1.42,-0.1958},
  {-2.5595,-0.62,-0.6764},
  {-2.5601,-0.62,-1.3188},
  {-2.7205,-1.42,-1.798},
  {-1.4401,-2.0599,-1.3193},
  {-1.4398,-2.0599,-1.1608},
  {-1.4396,-2.0599,-0.837},
  {-1.4398,-2.0599,-0.679},
  {-1.7592,-1.42,-0.3583},
  {-1.92,-0.9393,-0.679},
  {-1.9201,-0.94,-1.3203},
  {-1.7603,-1.42,-1.639},
  {-4.75,-4,-1.6389},
  {-5.5,-2.25,-1.635},
  {-5.75,-2,-0.037},
  {-5.25,-4,-0.1992},
  {-7.25,-3.5,-1},
  {-7.5,-3.5,-0.8365},
  {-7.5,-3.25,-0.6727},
  {-7.5,-3.5,-0.6727},
  {-2.3698,0.5619,-1.6373},
  {-2.4006,0.5613,-1.6376},
  {-2.529,0.5613,-1.9589},
  {-2.2093,0.5613,-1.9581},
  {-2.1536,0.551,0.1554},
  {-2.4006,0.551,0.1185},
  {-2.2388,0.5,-0.197},
  {-2.2388,0.5,-0.197},
  {-3.5628,-0.8286,-0.1351},
  {-4.48,-0.8284,-0.2014},
  {-4.3198,-1.1487,-0.8409},
  {-3.5194,-0.6687,-0.6738},
  {-3.52,-0.6636,-1.4795},
  {-4.3199,-1.1436,-1.1591},
  {-4.48,-0.8233,-1.7986},
  {-3.527,-0.8235,-1.8677},
  {-4.48,0.5649,-1.7986},
  {-4.6405,0.5649,-1.796},
  {-4.7195,0.5653,-2.0371},
  {-4.4062,0.5651,-2.1073},
  {-4.4785,0.5774,-0.034},
  {-4.7998,0.5775,-0.0396},
  {-4.6392,0.5773,-0.3554},
  {-4.6387,0.5773,-0.1933},
  {-2.3996,-4.1399,-0.6779},
  {-2.0795,-2.5399,-0.6785},
  {-1.1199,-3.3399,0.1198},
  {-2.3996,-4.1399,-1.3221},
  {-2.0799,-2.5399,-1.3191},
  {-1.1199,-3.3399,-2.1199},
  {-1.6016,-1.9547,-1.1596},
  {0.4081,-2.8247,-1.1594},
  {0.5,-2.75,-0.8409},
  {-1.6015,-1.9547,-0.8398},
  {-2.5995,-5.2239,-1.8488},
  {-1.7595,-3.9799,-2.1199},
  {-2.5996,-3.3039,-1.8489},
  {-2.5552,-5.2283,-0.1232},
  {-2.5551,-3.3083,-0.1232},
  {-1.759,-3.9483,-0.0389}
 },
 f={
  {16,15,14,13,12,11,10,9, c=7, uv={11,0.5,11.5,0,12.5,0,13,0.5,13,1.5,12.5,2,11.5,2,11,1.5} },
  {1,9,10,2, c=7, uv={12.5,0.5,12.5,1.5,11.5,1.5,11.5,0.5} },
  {3,11,12,4, c=7, uv={12.5,0.5,12.5,1.5,11.5,1.5,11.5,0.5} },
  {4,12,13,5, c=7, uv={12.5,0.5,12.5,1.5,11.5,1.5,11.5,0.5} },
  {6,14,15,7, c=7, uv={2.5,1.5,2.5,4.5,1.5,4.5,1.5,1.5} },
  {8,16,9,1, c=7, uv={12.5,0.5,12.5,1.5,11.5,1.5,11.5,0.5} },
  {17,1,2,18, c=7, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {18,2,3,19, c=7, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {19,3,4,20, c=7, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {20,4,5,21, c=7, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {21,5,6,22, c=7, uv={0.25,5.25,2.75,4.5,3,2.75,0.75,2.5} },
  {22,6,7,23, c=7, uv={15,0,16,0,16,1,15,1} },
  {23,7,8,24, c=7, uv={2.5,2.5,0.75,2.5,0.75,5,2.5,5} },
  {24,8,1,17, c=7, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {25,17,18,26, c=7, dbl=1, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {27,19,20,28, c=7, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {28,20,21,29, c=7, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {30,22,23,31, c=7, uv={1,2.75,1.25,1.5,3.5,1.5,3,3} },
  {32,24,17,25, c=7, uv={11.5,0,12.5,0,12.5,1,11.5,1} },
  {25,26,27,28,29,30,31,32, c=7, uv={0.5,5,1.25,5.75,3,5.75,3.25,4.75,3.5,3.75,2.75,2.5,1,2.75,0.5,4} },
  {33,2,10,34, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {34,10,11,35, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {35,11,3,36, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {36,3,2,33, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {37,33,34,38, c=7, dbl=1, uv={2.25,1.5,3,4.25,1,4.25,1.25,1.75} },
  {38,34,35,39, c=7, uv={2.25,1.5,2.75,4.25,0.75,4.5,1.25,1.5} },
  {39,35,36,40, c=7, dbl=1, uv={2.75,1.5,2.75,3.75,0.75,4.25,1,1.5} },
  {40,36,33,37, c=7, uv={3.25,2,3.25,4.25,0.75,3.75,1.75,1.75} },
  {37,38,39,40, c=7, uv={13.5,-5,13.5,-4,12.5,-4,12.5,-5} },
  {41,31,23,42, c=7, uv={2,2.25,2.25,5.25,0.5,5.5,0.75,2} },
  {42,23,24,43, c=7, uv={3.25,1.5,3.5,5.75,1,5.5,1.25,1.75} },
  {43,24,32,44, c=7, uv={2.25,1.75,2.75,6,0.75,6,1,1.75} },
  {44,32,31,41, c=7, uv={2.5,2,2.5,5.25,0.5,5.25,0.5,2.5} },
  {45,29,21,46, c=7, uv={2.75,1,2.5,6,0.5,6,0.75,1.25} },
  {46,21,22,47, c=7, uv={3.25,1.5,3,5.25,1,5.5,1.25,1.5} },
  {47,22,30,48, c=7, uv={2.75,1.75,2.75,5,1,5.5,1,1.5} },
  {48,30,29,45, c=7, uv={3.25,2,3.25,5,1.75,5,2,2} },
  {49,5,13,50, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {50,13,14,51, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {51,14,6,52, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {52,6,5,49, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {53,7,15,54, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {54,15,16,55, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {55,16,8,56, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {56,8,7,53, c=7, uv={12.5,0.5,13.5,0.5,13.5,1.5,12.5,1.5} },
  {57,53,54,58, c=7, uv={2,1.5,2.75,4,0.75,4.25,0.75,1.5} },
  {58,54,55,59, c=7, uv={1.75,1.25,2.5,4.25,0,4.25,0.5,1.25} },
  {59,55,56,60, c=7, uv={2,1.75,2.5,4.25,0.75,4.5,1,1.75} },
  {60,56,53,57, c=7, uv={2,2,2.25,4,0.5,3.75,1,2} },
  {61,49,50,62, c=7, uv={3,1.5,3,4,1,4,0.75,1.5} },
  {62,50,51,63, c=7, uv={2.75,1.25,3,4.25,1,4,1.25,1.25} },
  {63,51,52,64, c=7, uv={3,1.25,2.5,4.5,0.75,4.5,1,1.25} },
  {64,52,49,61, c=7, uv={2.25,1.25,3,4.25,1,4.25,1.25,1.25} },
  {65,68,69,66, c=10, uv={11,0,12,0,12,1,11,1} },
  {68,65,67,70, c=10, uv={11,1,13.5,1,13.75,3.25,10.75,3.25} },
  {71,69,70,72, c=10, uv={1,1.25,2.5,2,2.75,5,0.75,3.25} },
  {72,70,67,73, c=10, dbl=1, uv={11.75,4.5,11.75,2.75,13.25,3,13.25,4.5} },
  {73,67,66,74, c=10, uv={0.75,3.25,3.5,5.25,3,2,0.5,0.75} },
  {74,66,69,71, c=10, uv={11,0,12,0,12,1,11,1} },
  {71,72,73,74, c=10, uv={16,2,16,4.5,15,4.5,15,2} },
  {75,68,70,76, c=10, dbl=1, uv={0.5,1,0.75,4.25,3.5,4.5,3,2.75} },
  {76,70,69,77, c=10, dbl=1, uv={11,0,12,0,12,1,11,1} },
  {77,69,68,75, c=10, uv={11,0,12,0,12,1,11,1} },
  {75,76,77, c=10, uv={11.5,0,12,1,11,1} },
  {78,65,66,79, c=10, uv={11,0,12,0,12,1,11,1} },
  {79,66,67,80, c=10, dbl=1, uv={11,0,12,0,12,1,11,1} },
  {80,67,65,78, c=10, dbl=1, uv={0.5,1.5,0.75,4.5,3.5,4.5,2.5,1.75} },
  {78,79,80, c=10, uv={11.5,0,12,1,11,1} }
 }
}"#;
}
