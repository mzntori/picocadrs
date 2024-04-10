use crate::assets::point::Point2D;
use rlua::Error as LuaError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PicoError {
    #[error(r#"identifier is not "picocad""#)]
    Identifier,
    #[error("could not parse header field {0}")]
    HeaderField(String),
    #[error("found {0} header fields (expected 5)")]
    HeaderLength(usize),
    #[error("footer with lenght {0} (expected 15360)")]
    FooterLength(usize),
    #[error("found {0} uv-coordinates (expected {1})")]
    FaceUVMapLength(usize, usize),
    #[error("found {0} table elements (expected {1})")]
    TableLength(usize, usize),
    #[error("could not parse mesh field {0}")]
    MeshField(String),
    #[error("could not parse meshes from file")]
    MeshTable,
    #[error("could not split file properly ({0})")]
    Split(String),
    #[error("invalid vertex index")]
    Lua(#[from] LuaError),
    #[error("index out of range: {0:?} (expected < {1:?})")]
    IndexUSIZE(Point2D<usize>, Point2D<usize>),
}
