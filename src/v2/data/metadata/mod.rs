//! The metadata section of the project file holds any data that stretches across a whole project, like camera settings or export settings.

pub mod camera;
pub mod export_settings;

pub use camera::*;
pub use export_settings::*;

use serde::{Deserialize, Serialize};

/// Represents the metadata section of a picoCAD 2 project file.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Metadata {
    /// This is the version of the project file.
    /// As of now this can just be `"2.0"` and any other value will be overwritten once saved.
    version: String,
    /// Represents the mode used for shading the model.
    /// If `0` shadows are disabled, if `1` they are enabled.
    /// Other values don't have any functionality, can however load other sprites.
    shading_mode: i64,
    /// Represents the mode used for displaying faces of the model.
    /// If `0` only wires are shown, if `1` flat color is used, if `2` textures are used and if any other number is used all faces are being rendered in flat white.
    face_mode: i64,
    /// Represents the duration of the timeline in seconds.
    /// Setting this to anything below 0 crashes the game when clicking a mesh.
    /// Setting this to a number that is lower than already placed blocks doesn't delete them.
    motion_duration: f64,
    /// Represents the [`camera`](camera::Camera) section of the metadata.
    camera: Camera,
    /// Represents the [`export settings`](export_settings::ExportSettings) section of the metadata.
    export_settings: ExportSettings,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let foo: Metadata = serde_json::from_str(EXAMPLE).unwrap();
        let s = serde_json::to_string(&foo).unwrap();

        assert_eq!(foo, serde_json::from_str(&s).unwrap());
    }

    const EXAMPLE: &str = r#"
    {
      "motion_duration":2.5,
      "version":"2.0",
      "shading_mode":137,
      "face_mode":1,
      "export_settings":{
        "speed":15,
        "watermark2_color":4,
        "fov_type":"perspective",
        "outline_color":8,
        "dir":-1,
        "scale":0,
        "outline_size":0,
        "anim":"spin",
        "watermark":"picoCAD2",
        "watermark_color":6,
        "scanlines":true,
        "watermark2":"",
        "animate":false,
        "size":192
      },
      "camera":{
        "pos":{
          "y":4.794255386042,
          "z":7.3770415391672,
          "x":-4.7533539130606
        },
        "bookmark":{
          "pos":{
            "y":0,
            "z":0,
            "x":0
          },
          "distance_to_target":10,
          "target":{
            "y":0,
            "z":0,
            "x":0
          },
          "omega":-4.14,
          "theta":0.5
        },
        "distance_to_target":10,
        "target":{
          "y":0,
          "z":0,
          "x":0
        },
        "omega":-4.14,
        "theta":0.5
      }
    }"#;
}
