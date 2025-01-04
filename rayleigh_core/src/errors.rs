use thiserror::Error;

#[derive(Error, Debug)]
pub enum RayleighError {
    #[error("Unit mismatch")]
    UnitMismatch,
}
