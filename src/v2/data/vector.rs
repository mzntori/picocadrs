use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::axis::Axis;

/// General trait for both vector types.
/// Asserts certain things that both have to have in common.
pub trait Vector: Sized + Add + AddAssign + Sub + SubAssign {
    type Axis;

    /// Flattens the vector on some axis.
    fn flatten(&mut self, axis: Self::Axis);
    /// Normalizes the vector.
    fn normalize(&mut self);
    /// Scales the vector by some factor.
    fn scale(&mut self, factor: f64);
    /// Scales the vector so that the magnitude has the provided value.
    fn set_magnitude(&mut self, magnitude: f64);

    /// Flattens the vector on some axis and returns that vector.
    fn flattened(self, axis: Self::Axis) -> Self;
    /// Normalizes the vector and returns the normalized vector.
    fn normalized(self) -> Self;
    /// Scales the vector by some factor and returns the scaled vector.
    fn scaled(self, scale: f64) -> Self;
    /// Scales the vector so that the magnitude has the provided value and returns the scaled vector.
    fn with_magnitude(self, magnitude: f64) -> Self;

    /// Returns the vectors magnitude.
    fn magnitude(&self) -> f64;
}

/// Represents some sort of 2 dimensional position, vector or volume in space.
/// For the purpose of this crate, this is used to represent positions in 2D space, specifically on the UV-map.
#[derive(Debug, Default, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Vector2 {
    pub u: f64,
    pub v: f64,
}

impl Vector2 {
    /// Creates a new [`Vector2`].
    /// Takes the points u and v values as arguments.
    pub fn new(u: f64, v: f64) -> Vector2 {
        Vector2 { u, v }
    }
}

impl Vector for Vector2 {
    type Axis = Axis;

    /// Flattens the vector on the provided axis.
    fn flatten(&mut self, axis: Self::Axis) {
        match axis {
            Axis::U => self.u = 0.0,
            Axis::V => self.v = 0.0,
            _ => {}
        }
    }

    /// Normalizes the vector as good as possible.
    ///
    /// Since magnitude is not actually stored, but gets calculated per function call,
    /// the return value of [`magnitude`] can probably diverge somehow from the expected value, after calling this method.
    fn normalize(&mut self) {
        self.set_magnitude(1.0);
    }

    /// Scales the vector by the provided factor.
    fn scale(&mut self, factor: f64) {
        self.u *= factor;
        self.v *= factor;
    }

    /// Scales the vector, so that the magnitude of the vector is as close to the provided value as possible.
    ///
    /// Since magnitude is not actually stored, but gets calculated per function call,
    /// the return value of [`magnitude`] can probably diverge somehow from the expected value, after calling this method.
    fn set_magnitude(&mut self, magnitude: f64) {
        self.scale(magnitude / self.magnitude());
    }

    /// Consumes, then flattens the vector on the provided axis and returns the flattened vector.
    fn flattened(mut self, axis: Self::Axis) -> Self {
        match axis {
            Axis::U => self.u = 0.0,
            Axis::V => self.v = 0.0,
            _ => {}
        }

        return self;
    }

    /// Consumes, then normalizes the vector as good as possible and returns the normalized vector.
    ///
    /// Since magnitude is not actually stored, but gets calculated per function call,
    /// the return value of [`magnitude`] can probably diverge somehow from the expected value, after calling this method.
    fn normalized(self) -> Self {
        self.with_magnitude(1.0)
    }

    /// Consumes, then scales the vector by the provided factor and returns the scaled vectors.
    fn scaled(mut self, factor: f64) -> Self {
        self.u *= factor;
        self.v *= factor;

        return self;
    }

    /// Consumes, then scales the vector, so that the magnitude of the vector is as close to the provided
    /// value as possible and then returns the scaled vector.
    ///
    /// Since magnitude is not actually stored, but gets calculated per function call,
    /// the return value of [`magnitude`] can probably diverge somehow from the expected value, after calling this method.
    fn with_magnitude(self, magnitude: f64) -> Self {
        self.scaled(magnitude / self.magnitude())
    }

    /// *Calculates* the magnitude of the vector.
    fn magnitude(&self) -> f64 {
        (self.u.powi(2) + self.v.powi(2)).sqrt()
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            u: self.u + rhs.u,
            v: self.v + rhs.v,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.u += rhs.u;
        self.v += rhs.v;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            u: self.u - rhs.u,
            v: self.v - rhs.v,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.u -= rhs.u;
        self.v -= rhs.v;
    }
}

impl From<Vector3> for Vector2 {
    fn from(value: Vector3) -> Self {
        Vector2 {
            u: value.x,
            v: value.y,
        }
    }
}

/// Represents some sort of 3 dimensional position, vector or volume in space.
/// For the purpose of this crate, this is used to represent positions and scales in 3D space.
#[derive(Debug, Default, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Vector3 {
    pub z: f64,
    pub x: f64,
    pub y: f64,
}

impl Vector3 {
    /// Creates a new [`Vector3`].
    /// Takes the points x, y and z values as arguments.
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { z, x, y }
    }
}

impl Vector for Vector3 {
    type Axis = Axis;

    /// Flattens the vector on the provided axis.
    fn flatten(&mut self, axis: Self::Axis) {
        match axis {
            Axis::X => self.x = 0.0,
            Axis::Y => self.y = 0.0,
            Axis::Z => self.z = 0.0,
            _ => {}
        }
    }

    /// Normalizes the vector as good as possible.
    ///
    /// Since magnitude is not actually stored, but gets calculated per function call,
    /// the return value of [`magnitude`] can probably diverge somehow from the expected value, after calling this method.
    fn normalize(&mut self) {
        self.set_magnitude(1.0);
    }

    /// Scales the vector by the provided factor.
    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    /// Scales the vector, so that the magnitude of the vector is as close to the provided value as possible.
    ///
    /// Since magnitude is not actually stored, but gets calculated per function call,
    /// the return value of [`magnitude`] can probably diverge somehow from the expected value, after calling this method.
    fn set_magnitude(&mut self, magnitude: f64) {
        self.scale(magnitude / self.magnitude());
    }

    /// Consumes, then flattens the vector on the provided axis and returns the flattened vector.
    fn flattened(mut self, axis: Self::Axis) -> Self {
        match axis {
            Axis::X => self.x = 0.0,
            Axis::Y => self.y = 0.0,
            Axis::Z => self.z = 0.0,
            _ => {}
        }

        return self;
    }

    /// Consumes, then normalizes the vector as good as possible and returns the normalized vector.
    ///
    /// Since magnitude is not actually stored, but gets calculated per function call,
    /// the return value of [`magnitude`] can probably diverge somehow from the expected value, after calling this method.
    fn normalized(self) -> Self {
        self.with_magnitude(1.0)
    }

    /// Consumes, then scales the vector by the provided factor and returns the scaled vectors.
    fn scaled(mut self, factor: f64) -> Self {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;

        return self;
    }

    /// Consumes, then scales the vector, so that the magnitude of the vector is as close to the provided
    /// value as possible and then returns the scaled vector.
    ///
    /// Since magnitude is not actually stored, but gets calculated per function call,
    /// the return value of [`magnitude`] can probably diverge somehow from the expected value, after calling this method.
    fn with_magnitude(self, magnitude: f64) -> Self {
        self.scaled(magnitude / self.magnitude())
    }

    /// *Calculates* the magnitude of the vector.
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            z: self.z + rhs.z,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            z: self.z - rhs.z,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl From<Vector2> for Vector3 {
    fn from(value: Vector2) -> Self {
        Vector3 {
            z: 0.0,
            x: value.u,
            y: value.v,
        }
    }
}

#[macro_export]
/// Easy way to create a [`Vector2`] or [`Vector3`].
macro_rules! vector {
    ($u:expr, $v:expr) => {
        Vector2 { u: $u, v: $v }
    };
    ($x:expr, $y:expr, $z:expr) => {
        Vector3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f64_near;

    use super::*;

    #[test]
    fn test_vector2_flatten() {
        let mut v: Vector2 = Vector2::new(1.0, 2.0);
        v.flatten(Axis::V);

        assert_eq!(v, Vector2::new(1.0, 0.0));
        assert_eq!(v, Vector2::new(1.0, -1.0).flattened(Axis::V));
    }

    #[test]
    fn test_vector2_normalize() {
        let mut v: Vector2 = Vector2::new(3.0, 4.0);
        v.normalize();

        // assert_eq!(v, Vector2::new(0.6, 0.8));	This works but floats.
        assert_eq!(v, Vector2::new(6.0, 8.0).normalized());
    }

    #[test]
    fn test_vector2_scale() {
        let mut v: Vector2 = Vector2::new(-2.0, 1.0);
        v.scale(1.5);

        assert_eq!(v, Vector2::new(-3.0, 1.5));
        assert_eq!(v, Vector2::new(1.0, -0.5).scaled(-3.0));
    }

    #[test]
    fn test_vector2_set_magnitude() {
        let mut v: Vector2 = Vector2::new(3.0, 4.0);
        v.set_magnitude(1.0);

        // assert_eq!(v, Vector2::new(0.6, 0.8));	This works but floats.
        assert_eq!(v, Vector2::new(6.0, 8.0).with_magnitude(1.0));
        assert_eq!(v, Vector2::new(-6.0, -8.0).with_magnitude(-1.0));
    }

    #[test]
    fn test_vector2_magnitude() {
        let v: Vector2 = Vector2::new(3.0, 4.0);

        assert_f64_near!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector2_add() {
        let mut v: Vector2 = Vector2::new(1.0, 2.0) + Vector2::new(-2.5, 2.0);
        assert_eq!(v, Vector2::new(-1.5, 4.0));

        v += Vector2::new(1.0, -2.5);
        assert_eq!(v, Vector2::new(-0.5, 1.5));
    }

    #[test]
    fn test_vector2_sub() {
        let mut v: Vector2 = Vector2::new(1.0, 2.0) - Vector2::new(-2.5, 2.0);
        assert_eq!(v, Vector2::new(3.5, 0.0));

        v -= Vector2::new(1.0, -2.5);
        assert_eq!(v, Vector2::new(2.5, 2.5));
    }

    #[test]
    fn test_vector3_flatten() {
        let mut v: Vector3 = Vector3::new(1.0, 2.0, 3.0);
        v.flatten(Axis::Y);

        assert_eq!(v, Vector3::new(1.0, 0.0, 3.0));
        assert_eq!(v, Vector3::new(1.0, -1.0, 3.0).flattened(Axis::Y));
    }

    #[test]
    fn test_vector3_normalize() {
        let mut v: Vector3 = Vector3::new(3.0, 4.0, 2.0);
        v.normalize();

        assert_eq!(
            v,
            Vector3::new(0.5570860145311556, 0.7427813527082074, 0.3713906763541037)
        ); // This works but floats.
        assert_eq!(v, Vector3::new(6.0, 8.0, 4.0).normalized());
    }

    #[test]
    fn test_vector3_scale() {
        let mut v: Vector3 = Vector3::new(-2.0, 1.0, 2.0);
        v.scale(1.5);

        assert_eq!(v, Vector3::new(-3.0, 1.5, 3.0));
        assert_eq!(v, Vector3::new(1.0, -0.5, -1.0).scaled(-3.0));
    }

    #[test]
    fn test_vector3_set_magnitude() {
        let mut v: Vector3 = Vector3::new(3.0, 4.0, 2.0);
        v.set_magnitude(1.0);

        // assert_eq!(v, Vector3::new(0.6, 0.8));	This works but floats.
        assert_eq!(v, Vector3::new(6.0, 8.0, 4.0).with_magnitude(1.0));
        assert_eq!(v, Vector3::new(-6.0, -8.0, -4.0).with_magnitude(-1.0));
    }

    #[test]
    fn test_vector3_magnitude() {
        let v: Vector3 = Vector3::new(3.0, 4.0, 2.0);

        assert_f64_near!(v.magnitude(), 5.385164807134504);
    }

    #[test]
    fn test_vector3_add() {
        let mut v: Vector3 = Vector3::new(1.0, 2.0, 3.0) + Vector3::new(-2.5, 2.0, -1.0);
        assert_eq!(v, Vector3::new(-1.5, 4.0, 2.0));

        v += Vector3::new(1.0, -2.5, 0.5);
        assert_eq!(v, Vector3::new(-0.5, 1.5, 2.5));
    }

    #[test]
    fn test_vector3_sub() {
        let mut v: Vector3 = Vector3::new(1.0, 2.0, 3.0) - Vector3::new(-2.5, 2.0, -1.0);
        assert_eq!(v, Vector3::new(3.5, 0.0, 4.0));

        v -= Vector3::new(1.0, -2.5, 2.0);
        assert_eq!(v, Vector3::new(2.5, 2.5, 2.0));
    }

    #[test]
    fn test_vector_macro() {
        assert_eq!(vector!(1.0, 2.0), Vector2::new(1.0, 2.0));
        assert_eq!(vector!(1.0, 2.0, -3.0), Vector3::new(1.0, 2.0, -3.0));
    }
}
