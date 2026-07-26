use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use arrayvec::ArrayString;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct EnchantmentKind(ArrayString<16>);

impl EnchantmentKind {
    pub fn from<'a>(id: impl Into<&'a str>) -> Self {
        let id = ArrayString::from(id.into())
            .expect("EnchantmentKind can't have an ID longer than 16 characters");

        Self(id)
    }
}

impl Display for EnchantmentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
