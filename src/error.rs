use thiserror::Error;

#[derive(Debug, Error)]
pub enum PicoParseError {
    #[error(r#"identifier is not "picocad""#)]
    Identifier,
    #[error("could not parse header field {0}")]
    HeaderField(String),
    #[error("found {0} header fields (expected 5)")]
    HeaderLength(usize),
}
