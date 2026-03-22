use serde::{Deserialize, Serialize};

use crate::v2::data::point::Point;

/// Represents camera settings.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Camera {
    /// Represents the "vertical" angle of the camera.
    /// The valid value range for this is `-pi/2` to `pi/2` and any value out side of that is clamped.
    /// At `pi/2` the camera sees the model from exactly above, at `-pi/2' from below and at `0.0` from the side.
    pub theta: f64,
    /// Represents the "horizontal" angle of the camera.
    /// If [`theta`](Self::theta) and this field are `0.0` the camera looks exactly along the x-axis.
    /// Negative value rotate the camera to the left while positive values rotate the camera to the right.
    /// One whole rotation is equal to `2 * pi`.
    /// This means an `omega` of `1` is equal to `2 * pi + 1` or also `-(2 * pi - 1)`.
    pub omega: f64,
    /// Represents the distance from the camera.
    /// This can be set to negative values, which results in some interesting results.
    pub distance_to_target: f64,
    /// Represents the absolute position of the camera.
    /// This value is automatically adjusted by picoCAD when moving the camera around.
    /// Changing this value manually will not do anything as it will be overwritten once the project is loaded.
    pub pos: Point,
    /// Represents the position the camera targets aka. rotates around.
    pub target: Point,
    /// Represents the camera settings that are loaded when the user presses the bookmark icon.
    /// The bookmark camera can have a bookmark itself although it doesn't serve any purpose and is truncated when saving the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmark: Option<Box<Camera>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let d: Camera = serde_json::from_str(EXAMPLE).unwrap();
        let s = serde_json::to_string(&d).unwrap();

        assert_eq!(
            EXAMPLE
                .to_string()
                .replace("\n", "")
                .replace("\t", "")
                .replace(" ", ""),
            s
        );
    }

    const EXAMPLE: &str = r#"
    {
      "pos":{
        "z":7.3770415391672,
        "x":-4.7533539130606,
        "y":4.794255386042
      },
      "target":{
        "z":0,
        "x":0,
        "y":0
      },
      "omega":-4.14,
      "theta":0.5,
      "bookmark":{
        "pos":{
          "z":0,
          "x":0,
          "y":0
        },
        "target":{
          "z":0,
          "x":0,
          "y":0
        },
        "omega":-4.14,
        "theta":0.5,
        "distance_to_target":10
      },
      "distance_to_target":10
    }"#;
}
