use std::f32::consts::PI;
use rlua::{Lua, Table};
use rlua::prelude::LuaError;

use crate::assets::Serialize;

/// A vector containing 3 float values representing x, y and z.
#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// Creates a new Vector that points to 0|0|0.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Adds the given arguments to the corresponding components of this Vector.
    pub fn add(&mut self, x: f32, y: f32, z: f32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    /// Adds the components of a Vector to this Vector.
    pub fn add_vector(&mut self, v: &Vector) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }

    /// Multiplies all components of this Vector by the given factor.
    pub fn scale(&mut self, f: f32) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }

    /// Multiplies the components with the corresponding components of the given Vector.
    pub fn scale_with(&mut self, v: &Vector) {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
    }

    /// Transforms the Vector into a 2D Vector by setting the z component to 0.
    pub fn flatten(&mut self) {
        self.z = 0.0;
    }

    /// Returns the amount of the Vector.
    pub fn amount(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        self.scale(1.0 / self.amount())
    }

    /// Rounds the components of the Vector to the 2nd digit after the comma.
    pub fn round(&mut self) {
        self.x = (self.x * 100.0).round() / 100.0;
        self.y = (self.y * 100.0).round() / 100.0;
        self.z = (self.z * 100.0).round() / 100.0;
    }

    /// Rotates this Vector around the x-axis, then the y-axis and then z-axis.
    /// The Vector provided in the arguments is being used as measure of rotation, where 0.0 means no rotation and 1.0 means a full rotation.
    ///
    /// In this example `v1` gets rotated around the x-axis by 90 degree.
    /// ```rust
    ///  use picocadrs::assets::Vector;
    ///
    ///  let mut v1 = Vector::new(3.0, 3.0, 2.0);
    ///  let v2 = Vector::new(3.0, -2.0, 3.0);
    ///
    ///  v1.rotate(&Vector::new(0.25, 0.0, 0.0));
    ///  v1.round();
    ///  assert_eq!(v1, v2)
    /// ```
    pub fn rotate(&mut self, rotv: &Vector) {
        let mut rot_vect: Vector = rotv.clone();

        rot_vect.scale(2.0 * PI);

        // Rotate around x
        let from_vect: Vector = self.clone();
        self.y = rot_vect.x.cos() * from_vect.y - rot_vect.x.sin() * from_vect.z;
        self.z = rot_vect.x.sin() * from_vect.y + rot_vect.x.cos() * from_vect.z;

        // Rotate around y
        let from_vect: Vector = self.clone();
        self.z = rot_vect.y.cos() * from_vect.z - rot_vect.y.sin() * from_vect.x;
        self.x = rot_vect.y.sin() * from_vect.z + rot_vect.y.cos() * from_vect.x;

        // Rotate around z
        let from_vect: Vector = self.clone();
        self.x = rot_vect.z.cos() * from_vect.x - rot_vect.z.sin() * from_vect.y;
        self.y = rot_vect.z.sin() * from_vect.x + rot_vect.z.cos() * from_vect.y;
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self { x: 0f32, y: 0f32, z: 0f32 }
    }
}

impl From<Table<'_>> for Vector {
    fn from(t: Table) -> Self {
        let vector_values: Vec<Result<f32, LuaError>> = t.sequence_values::<f32>().collect();

        Self {
            x: vector_values.get(0).unwrap().clone().unwrap(),
            y: vector_values.get(1).unwrap().clone().unwrap(),
            z: vector_values.get(2).unwrap().clone().unwrap(),
        }
    }
}

impl From<String> for Vector {
    fn from(s: String) -> Self {
        let mut v: Vector = Vector::default();
        let lua = Lua::new();
        lua.context(|ctx| {
            let table: Table = ctx.load(s.as_str()).eval().expect("Failed to parse Vector");
            v = Vector::from(table);
        });
        v
    }
}

impl From<&str> for Vector {
    fn from(s: &str) -> Self {
        Vector::from(s.to_string())
    }
}

impl Serialize for Vector {
    fn serialize(&self) -> String {
        format!("{}{},{},{}{}", "{", &self.x, &self.y, &self.z, "}")
    }
}
