use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use arrayvec::ArrayString;

use crate::id::id_from;
use crate::id::IdLengthError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct EnchantmentKind(ArrayString<16>);

impl EnchantmentKind {
    pub fn from<'a>(id: impl AsRef<str>) -> Result<Self, IdLengthError> {
        id_from(id, |id| Self(id))
    }
}

impl Display for EnchantmentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
