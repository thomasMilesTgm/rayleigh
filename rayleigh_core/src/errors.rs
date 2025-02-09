use thiserror::Error;

#[derive(Error, Debug)]
pub enum RayleighError {
    #[error("Unit mismatch")]
    UnitMismatch,

    #[error("Cast Failed {from} -> {to}")]
    CastFailed { from: String, to: String },
}
