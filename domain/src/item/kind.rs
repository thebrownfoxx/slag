use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use arrayvec::ArrayString;

use crate::id::IdLengthError;
use crate::id::id_from;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct ItemKind(ArrayString<16>);

impl ItemKind {
    pub fn from<'a>(id: impl AsRef<str>) -> Result<Self, IdLengthError> {
        id_from(id, |id| Self(id))
    }
}

impl Display for ItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
