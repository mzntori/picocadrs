//! For different kind of coordinates.
//!
//! This module houses the structs `Point2D` and `Point3D` that describe points in either 2- or
//! 3-dimensional space.

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// Represents a 2-dimensional point in space.
/// In this crates context used for uv-mapping.
///
/// It can be either created by using the method `new` or by using the `uv!` macro.
///
/// # Example
///
/// ```
/// use picocadrs::assets::point::Point2D;
/// use picocadrs::uv;
///
/// let mut point = Point2D::new(2, 4);
///
/// assert_eq!(point.u, 2);
/// assert_eq!(point.v, 4);
///
/// assert_eq!(point, uv!(2, 4));
///
/// point.set(1, 2);
/// assert_eq!(point, uv!(1, 2));
///
/// assert_eq!(point + point, uv!(2, 4));
/// assert_eq!(point - point, uv!(0, 0));
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point2D<T> {
    pub u: T,
    pub v: T,
}

impl<T> Point2D<T> {
    /// Used to create new points in a 2-dimensional space.
    /// Takes the points `u` and `v` coordinates as arguments.
    ///
    /// A simpler way to create new `Point2D`s is to use the `uv!` macro.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point2D;
    /// use picocadrs::uv;
    ///
    /// let point = Point2D::new(2, 4);
    ///
    /// assert_eq!(point.u, 2);
    /// assert_eq!(point.v, 4);
    ///
    /// assert_eq!(point, uv!(2, 4))
    /// ```
    pub fn new(u: T, v: T) -> Point2D<T> {
        Point2D { u, v }
    }

    /// Sets the points coordinates to the ones given.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point2D;
    ///
    /// let mut point = Point2D::new(2, 4);
    ///
    /// assert_eq!(point.u, 2);
    /// assert_eq!(point.v, 4);
    ///
    /// point.set(3, 4);
    ///
    /// assert_eq!(point.u, 3);
    /// assert_eq!(point.v, 4);
    /// ```
    pub fn set(&mut self, u: T, v: T) {
        self.u = u;
        self.v = v;
    }

    /// Used to apply functions on every coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point2D;
    ///
    /// let mut point = Point2D::new(2, 3);
    ///
    /// assert_eq!(point, Point2D::new(2, 3));
    /// point.map(|c| {
    ///    c * 2
    /// });
    /// assert_eq!(point, Point2D::new(4, 6));
    ///
    /// ```
    pub fn map(&mut self, f: fn(&T) -> T) {
        self.u = f(&self.u);
        self.v = f(&self.v);
    }
}

impl<T: Add<Output = T>> Add for Point2D<T> {
    type Output = Point2D<T>;

    /// Adds points together.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point2D;
    ///
    /// let p1 = Point2D::new(1, 4);
    /// let p2 = Point2D::new(2, 1);
    ///
    /// assert_eq!(p1 + p2, Point2D::new(3, 5));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Point2D {
            u: self.u + rhs.u,
            v: self.v + rhs.v,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point2D<T> {
    type Output = Point2D<T>;

    /// Subtracts points from each other.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point2D;
    ///
    /// let p1 = Point2D::new(1, 4);
    /// let p2 = Point2D::new(2, 1);
    ///
    /// assert_eq!(p1 - p2, Point2D::new(-1, 3));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {
            u: self.u - rhs.u,
            v: self.v - rhs.v,
        }
    }
}

impl<T: Display> Display for Point2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.u, self.v)
    }
}

/// Easier way to create a `Point2D`.
///
/// # Example
///
/// ```
/// use picocadrs::assets::point::Point2D;
/// use picocadrs::uv;
///
/// assert_eq!(uv!(2, 3), Point2D::new(2, 3));
/// assert_eq!(uv!(4, -3), Point2D::new(4, -3));
/// ```
#[macro_export]
macro_rules! uv {
    ($u:expr, $v:expr) => {
        Point2D { u: $u, v: $v }
    };
}

/// Represents a 3-dimensional point in space.
/// In this crates context mostly used for displaying points of vertices.
///
/// It can be either created by using the method `new` or by using the `xyz!` macro.
///
/// # Example
///
/// ```
/// use picocadrs::assets::point::Point3D;
/// use picocadrs::xyz;
///
/// let mut point = Point3D::new(2, 4, -1);
///
/// assert_eq!(point.x, 2);
/// assert_eq!(point.y, 4);
/// assert_eq!(point.z, -1);
///
/// assert_eq!(point, xyz!(2, 4, -1));
///
/// point.set(1, 2, 3);
/// assert_eq!(point, xyz!(1, 2, 3));
///
/// assert_eq!(point + point, xyz!(2, 4, 6));
/// assert_eq!(point - point, xyz!(0, 0, 0));
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3D<T> {
    /// Used to create new points in a 3-dimensional space.
    /// Takes the points `x`, `y` and `z` coordinates as arguments.
    ///
    /// A simpler way to create new `Point3D`s is to use the `xyz!` macro.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point3D;
    /// use picocadrs::xyz;
    ///
    /// let point = Point3D::new(2, 4, -1);
    ///
    /// assert_eq!(point.x, 2);
    /// assert_eq!(point.y, 4);
    /// assert_eq!(point.z, -1);
    ///
    /// assert_eq!(point, xyz!(2, 4, -1));
    /// ```
    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D { x, y, z }
    }

    /// Sets the points coordinates to the ones given.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point3D;
    ///
    /// let mut point = Point3D::new(2, 4, -1);
    ///
    /// assert_eq!(point.x, 2);
    /// assert_eq!(point.y, 4);
    /// assert_eq!(point.z, -1);
    ///
    /// point.set(-3, 4, 2);
    ///
    /// assert_eq!(point.x, -3);
    /// assert_eq!(point.y, 4);
    /// assert_eq!(point.z, 2);
    /// ```
    pub fn set(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Used to apply functions on every coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point3D;
    ///
    /// let mut point = Point3D::new(2, 3, -1);
    ///
    /// assert_eq!(point, Point3D::new(2, 3, -1));
    /// point.map(|c| {
    ///    c * 2
    /// });
    /// assert_eq!(point, Point3D::new(4, 6, -2));
    ///
    /// ```
    pub fn map(&mut self, f: fn(&T) -> T) {
        self.x = f(&self.x);
        self.y = f(&self.y);
        self.z = f(&self.z);
    }
}

impl<T: Add<Output = T>> Add for Point3D<T> {
    type Output = Point3D<T>;

    /// Adds points together.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point3D;
    ///
    /// let p1 = Point3D::new(1, 4, -2);
    /// let p2 = Point3D::new(2, -2, 3);
    ///
    /// assert_eq!(p1 + p2, Point3D::new(3, 2, 1));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point3D<T> {
    type Output = Point3D<T>;

    /// Subtracts points from each other.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::point::Point3D;
    ///
    /// let p1 = Point3D::new(1, 4, 2);
    /// let p2 = Point3D::new(2, 1, -4);
    ///
    /// assert_eq!(p1 - p2, Point3D::new(-1, 3, 6));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Display> Display for Point3D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

/// Easier way to create a `Point3D`.
///
/// # Example
///
/// ```
/// use picocadrs::assets::point::Point3D;
/// use picocadrs::xyz;
///
/// assert_eq!(xyz!(2, 3, -1), Point3D::new(2, 3, -1));
/// assert_eq!(xyz!(4, -3, 0), Point3D::new(4, -3, 0));
/// ```
#[macro_export]
macro_rules! xyz {
    ($x:expr, $y:expr, $z:expr) => {
        Point3D {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_uv_new() {
        let point = Point2D::new(2, 4);

        assert_eq!(point.u, 2);
        assert_eq!(point.v, 4);
    }

    #[test]
    fn test_uv_set() {
        let mut point = Point2D::new(2, 4);

        assert_eq!(point.u, 2);
        assert_eq!(point.v, 4);

        point.set(3, 4);

        assert_eq!(point.u, 3);
        assert_eq!(point.v, 4);
    }

    #[test]
    fn test_uv_add() {
        let p1 = Point2D::new(1, 4);
        let p2 = Point2D::new(2, 1);

        assert_eq!(p1 + p2, Point2D::new(3, 5));
    }

    #[test]
    fn test_uv_sub() {
        let p1 = Point2D::new(1, 4);
        let p2 = Point2D::new(2, 1);

        assert_eq!(p1 - p2, Point2D::new(-1, 3));
    }

    #[test]
    fn test_uv_macro() {
        assert_eq!(uv!(2, 3), Point2D::new(2, 3));
        assert_eq!(uv!(4, -3), Point2D::new(4, -3));
    }

    #[test]
    fn test_uv_map() {
        let mut point = Point2D::new(2, 3);

        assert_eq!(point, uv!(2, 3));
        point.map(|v| v * 2);
        assert_eq!(point, uv!(4, 6));
    }

    #[test]
    fn test_uv_tostring() {
        let p = Point2D::new(2, 4);

        assert_eq!("2,4", p.to_string());
    }

    #[test]
    fn test_xyz_new() {
        let point = Point3D::new(2, 4, -1);

        assert_eq!(point.x, 2);
        assert_eq!(point.y, 4);
        assert_eq!(point.z, -1);
    }

    #[test]
    fn test_xyz_set() {
        let mut point = Point3D::new(2, 4, -1);

        assert_eq!(point.x, 2);
        assert_eq!(point.y, 4);
        assert_eq!(point.z, -1);

        point.set(-3, 4, 2);

        assert_eq!(point.x, -3);
        assert_eq!(point.y, 4);
        assert_eq!(point.z, 2);
    }

    #[test]
    fn test_xyz_add() {
        let p1 = Point3D::new(1, 4, -2);
        let p2 = Point3D::new(2, -2, 3);

        assert_eq!(p1 + p2, Point3D::new(3, 2, 1));
    }

    #[test]
    fn test_xyz_sub() {
        let p1 = Point3D::new(1, 4, 2);
        let p2 = Point3D::new(2, 1, -4);

        assert_eq!(p1 - p2, Point3D::new(-1, 3, 6));
    }

    #[test]
    fn test_xyz_macro() {
        assert_eq!(xyz!(2, 3, -1), Point3D::new(2, 3, -1));
        assert_eq!(xyz!(4, -3, 0), Point3D::new(4, -3, 0));
    }

    #[test]
    fn test_xyz_map() {
        let mut point = Point3D::new(2, 3, -1);

        assert_eq!(point, Point3D::new(2, 3, -1));
        point.map(|c| c * 2);
        assert_eq!(point, Point3D::new(4, 6, -2));
    }

    #[test]
    fn test_xyz_tostring() {
        let point = Point3D::new(2, 3, -1);

        assert_eq!("2,3,-1", point.to_string());
    }
}
