use crate::assets::point::Point2D;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PicoParseError {
    #[error(r#"identifier is not "picocad""#)]
    Identifier,
    #[error("could not parse header field {0}")]
    HeaderField(String),
    #[error("found {0} header fields (expected 5)")]
    HeaderLength(usize),
    #[error("footer with lenght {0} (expected 15360)")]
    FooterLength(usize),
}

#[derive(Debug, Error)]
pub enum PicoError {
    #[error("index out of range: {0:?} (expected < {1:?})")]
    IndexUSIZE(Point2D<usize>, Point2D<usize>),
}
