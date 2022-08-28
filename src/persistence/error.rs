use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Already exists")]
    AlreadyExistsError,

    #[error("Not found")]
    NotFoundError,
}
