use diesel::result;

type DieselError = result::Error;

#[derive(Debug)]
pub enum Error {
    ValidationError,
    InternalServerError,
}

impl From<DieselError> for Error {
    fn from(_: DieselError) -> Self {
        Error::InternalServerError
    }
}
