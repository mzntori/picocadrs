/// Represents an axis used in picoCAD.
///
/// Note that `U != X` and `V != Y` even if they might be treated similarly elsewhere.
/// Instead use convertion methods isntead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// u axis on the UV-map
    U,
    /// v axis on the UV-map
    V,
    /// x axis in 3D space
    X,
    /// y axis in 3D space
    Y,
    /// z axis in 3D space
    Z,
}

impl Axis {
    /// Converts the axis into a xyz axis.
    ///
    /// This means `U` and `V` get converted into `X` and `Y` respectively.
    /// Any axis already in xyz stays the same.
    pub fn into_xyz(self) -> Axis {
        match self {
            Axis::U => Axis::X,
            Axis::V => Axis::Y,
            _ => self,
        }
    }

    /// Converts the axis into a uv axis.
    ///
    /// This means `X` and `Y` get converted into `U` and `V` respectively.
    /// `Z` however returns none.
    /// Any axis already in uv stays the same.
    pub fn into_uv(self) -> Option<Axis> {
        match self {
            Axis::X => Some(Axis::U),
            Axis::Y => Some(Axis::V),
            Axis::Z => None,
            _ => Some(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis_convertion() {
        assert_eq!(Axis::U, Axis::X.into_uv().unwrap());
        assert_eq!(Axis::V, Axis::Y.into_uv().unwrap());
        assert_eq!(None, Axis::Z.into_uv());

        assert_eq!(Axis::X, Axis::U.into_xyz());
        assert_eq!(Axis::Y, Axis::V.into_xyz());
    }
}
