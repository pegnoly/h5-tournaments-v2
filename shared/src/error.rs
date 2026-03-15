use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedError {
    #[error(transparent)]
    TonicCodegen(#[from] tonic::codegen::StdError)
}