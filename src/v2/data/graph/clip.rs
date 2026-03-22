//! This module holds any data used for clips.
//! Clips are small pieces that transform a mesh inside of an animation.

// TODO

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Curve {
    Linear,
    EaseIn,
    EaseOut,
    Soft,
    Elastic,
    Bounce,
    Instant,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Clip {
    delta: Option<f64>,
}
