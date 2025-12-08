use crate::error::{Error, ErrorKind};

pub fn apply<T: Required>(v: &T, _: ()) -> Result<(), Error> {
    if !v.is_set() {
        return Err(Error::from_kind(ErrorKind::Required));
    }
    Ok(())
}

pub trait Required {
    fn is_set(&self) -> bool;
}

impl<T> Required for Option<T> {
    fn is_set(&self) -> bool {
        self.is_some()
    }
}
