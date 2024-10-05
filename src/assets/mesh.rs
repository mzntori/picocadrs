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

use crate::{
    assets::{Face, Point3D},
    error::PicoError,
    point,
};
use rlua::{Lua, Table, Value};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use crate::assets::edge::Edge;

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
        let mut left = self.clone();
        let mut right = other.clone();

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

    // TODO: tests, docs
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

    #[cfg(feature = "svg")]
    pub fn svg_path_data(&self, angle: SVGAngle, scale: f64) -> Data {
        let data = Data::new();



        data
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
