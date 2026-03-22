//! This module holds all datastructures required to store the actual 3d model.
//! The reason it is called graph is that it is a Tree structure where each node can have a mesh, animations, a transform and individual various settings.
//! Nodes that do not have their own mesh kind of act like a folder but there also is a seperate flag for that.

pub mod clip;
pub mod face;
pub mod mesh;
pub mod transform;
