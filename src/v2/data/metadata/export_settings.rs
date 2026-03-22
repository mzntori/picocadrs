//! This module holds any settings for exporting a model into a GIF.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents the different types of motion the camera can perform on export.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Animation {
    /// Spins the camera in a circular motion around the model.
    Spin,
    /// Sways teh camera from side to side with easing on each end.
    Sway,
    /// Sways teh camera from side to side without easing on each end.
    PingPong,
    /// Camera is fixed in space.
    Fixed,
}

/// Represents the three types of projection.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FovType {
    Perspective,
    Orthographic,
    Fisheye,
}

/// Represents the direction of the camera.
#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, PartialEq)]
#[repr(i64)]
pub enum Direction {
    Left = -1,
    Right = 1,
}

/// Represents the export settings.
/// Any field that doesn't state that it is set to a default value if the value is invalid when entering export mode, crashes the program when entering export mode.
/// Represents the export settings.
/// Any field that is not wrapped inside an [`Option`](Option) crashes the program when omitted when entering export mode.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ExportSettings {
    /// Represents the the animation type of the camera.
    pub anim: Animation,
    /// Represents the direction the camera rotates when [`anim`](Self::anim) is set to [`Spin`](Animation::Spin).
    /// In the project file this is represented by an integer where `-1` equals clockwise rotation of the camera aroudn the model and `1` counterclockwise.
    /// If this field is set to something other than these two when exporting it defaults to `-1`.
    pub dir: Direction,
    /// Determines weither or not to use the custom animation.
    /// If this field is omitted, when entering export mode it is set to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate: Option<bool>,
    /// Represents the amount of time in seconds the gif will be after rendering (aka is represented by the "Duration" field in picoCAD itself).
    /// If [`animate`](Self::animate) is `true` this field will automatically take the the value of `motion_duration` TODO.
    /// If this field is not set to either `2`, `5`, `10` or `15` it is set to `15` when entering export mode.
    pub speed: u64,
    /// Represents outline thickness on pixels.
    /// If this field is set to `0` no outlines will be rendered.
    /// If this field is omitted when entering export mode it is set to `1`.
    /// If this field is set to some thin `> 4` it is automatically set to 0 when entering export mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<u64>,
    /// Represents the colorpalette index for the outline.
    /// If this field is omitted, when entering export mode it is set to `8`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<usize>,
    /// Determines weither or not to render scanlines.
    /// If this field is omitted, when entering export mode it is set to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanlines: Option<bool>,
    /// Represents the colorpalette index for the scanlines.
    /// If this field is omitted, picoCAD defaults to using `1` but isn't saved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanline_color: Option<usize>,
    /// Represents the type of projection used for rendering.
    /// If omitted or set to an invalid value, defaults to `"perspective"` when entering export mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fov_type: Option<FovType>,
    /// Represents the water mark text in the bottom left.
    pub watermark2: String,
    /// Represents the colorpalatte index for the water mark in the bottom left.
    /// If this field is omitted, picoCAD defaults to using `1` but isn't saved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark2_color: Option<usize>,
    /// Represents the water mark text in the bottom right.
    pub watermark: String,
    /// Represents the colorpalatte index for the water mark in the bottom right.
    /// If this field is omitted, picoCAD defaults to using `1` but isn't saved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark_color: Option<usize>,
    /// Represents the edge length in pixels.
    /// If this field is not set to `64`, `128` or `192` it is set to `192` when entering export mode.
    pub size: u64,
    /// Represents the scale factor when rendering.
    /// For example if this is set to `3` each pixel inprogram will be rendered as 3x3 pixels in the gif.
    /// If this is set to `0` or anything `> 5`, defaults to `1` when entering export mode.
    pub scale: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let d: ExportSettings = serde_json::from_str(EXAMPLE).unwrap();
        let s = serde_json::to_string(&d).unwrap();

        assert_eq!(EXAMPLE.to_string().replace("\n", "").replace("\t", ""), s);
    }

    const EXAMPLE: &str = r#"{
	"anim":"spin",
	"dir":1,
	"animate":false,
	"speed":15,
	"outline_size":0,
	"outline_color":8,
	"scanlines":true,
	"scanline_color":4,
	"fov_type":"perspective",
	"watermark2":"",
	"watermark2_color":4,
	"watermark":"picoCAD2",
	"watermark_color":2,
	"size":192,
	"scale":5
	}"#;
}
