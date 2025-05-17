use measurements::Angle;

use super::vector::{Vector, Vector3};

/// Represents a locked picoCAD (2) camera.
/// Locked in this case means that all values that depend on other values stored are guaranteed to be accurate in relation to eachother.
///
///	If you want a camera that does not have these restrictions, consider using [`UnlockedCamera`].
///
/// # Assumptions
///
/// - `pos` is relative to target.
#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub struct Camera {
    target: Vector3,
    #[serde(rename = "distance_to_target")]
    magnitude: f64,
    #[serde(rename = "pos")]
    position: Vector3,
    theta: Angle,
    omega: Angle,
}

impl Camera {
    /// Calculates the current magnitude.
    fn calculate_magnitude(&self) -> f64 {
        self.position.magnitude()
    }

    /// Calculates omega based on current position.
    fn calculate_omega(&self) -> Angle {
        Angle::from_radians((self.position.z / self.position.x).atan())
    }

    /// Calculates theta based on current position.
    fn calculate_theta(&self) -> Angle {
        Angle::from_radians((self.position.y / self.position.magnitude()).asin())
    }

    /// Creates a new camera based on target and position.
    pub fn new(target: Vector3, position: Vector3) -> Camera {
        let mut cam = Camera {
            target,
            magnitude: 0.0,
            position,
            theta: Angle::from_radians(0.0),
            omega: Angle::from_radians(0.0),
        };

        cam.position = position;
        cam.update_from_position();

        cam
    }

    /// Updates magnitude, theta and omega based on position.
    /// Acts as a reverse operation of [`update_from_angles_and_magnitude`].
    pub fn update_from_position(&mut self) {
        self.magnitude = self.calculate_magnitude();
        self.omega = self.calculate_omega();
        self.theta = self.calculate_theta();
    }

    /// Updates position based on magnitude, theta and omega.
    /// Acts as a reverse operation of [`update_from_position`].
    pub fn update_from_angles_and_magnitude(&mut self) {
        self.position.y = self.magnitude * self.theta.sin();
        let a = (self.magnitude.powi(2) - self.position.y.powi(2)).sqrt();
        self.position.z = a * self.omega.sin();
        self.position.x = a * self.omega.cos();
    }

    /// Borrows the target position.
    ///
    /// In a project file this is stored in the `target` field.
    pub fn target(&self) -> &Vector3 {
        &self.target
    }

    /// Borrows the target position mutable.
    ///
    /// In a project file this is stored in the `target` field.
    pub fn target_mut(&mut self) -> &mut Vector3 {
        &mut self.target
    }

    /// Borrows the cameras relative position to [`target`].
    ///
    /// In a project file this is stored in the `pos` field.
    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    /// Borrows the magnitude.
    /// If the camera has not been created from an [`UnlockedCamera`], this is always equal to the magnitude of [`position`].
    ///
    /// In a project file this is stored in the `distance_to_target` field.
    pub fn magnitude(&self) -> &f64 {
        &self.magnitude
    }

    /// Borrows the angle theta.
    /// If the camera has not been created from an [`UnlockedCamera`], this is accurate to the angle of the magnitude of [`position`].
    ///
    /// In a project file this is stored in the `theta` field.
    pub fn theta(&self) -> &Angle {
        &self.theta
    }

    /// Borrows the angle omega.
    /// If the camera has not been created from an [`UnlockedCamera`], this is accurate to the angle of the magnitude of [`position`].
    ///
    /// In a project file this is stored in the `omega` field.
    pub fn omega(&self) -> &Angle {
        &self.omega
    }

    /// Sets the target of the camera to the provided value.
    pub fn set_target(&mut self, new: Vector3) {
        self.target = new;
    }

    /// Sets the position of the camera to the provided value.
    pub fn set_position(&mut self, new: Vector3) {
        self.target = new;
        self.update_from_position();
    }

    /// Sets the magnitude of the camera to the provided value.
    pub fn set_magnitude(&mut self, new: f64) {
        self.magnitude = new;
        self.update_from_angles_and_magnitude();
    }

    /// Sets the theta of the camera to the provided value.
    pub fn set_theta(&mut self, new: Angle) {
        self.theta = new;
        self.update_from_angles_and_magnitude();
    }

    /// Sets omega of the camera to the provided value.
    pub fn set_omega(&mut self, new: Angle) {
        self.omega = new;
        self.update_from_angles_and_magnitude();
    }
}

pub struct UnlockedCamera;

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    use crate::vector;

    use super::*;

    const DEFAULT_POS: Vector3 = vector!(2.026658184747, 6.5516349516249, 3.746506626054);
    const DEFAULT_MAG: f64 = 7.8145745780703;

    #[test]
    fn camera_magnitude() {
        let c = Camera::new(Vector3::default(), DEFAULT_POS);

        assert_float_absolute_eq!(DEFAULT_MAG, *c.magnitude());
        assert_float_absolute_eq!(DEFAULT_MAG + 1.0, *c.magnitude() + 1.0);
    }
}
