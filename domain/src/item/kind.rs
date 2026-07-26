use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use arrayvec::ArrayString;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct ItemKind(ArrayString<16>);

impl ItemKind {
    pub fn from<'a>(value: impl Into<&'a str>) -> Self {
        let id = ArrayString::from(value.into())
            .expect("ItemKind can't have an ID longer than 16 characters");

        Self(id)
    }
}

impl Display for ItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
