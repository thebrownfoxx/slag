use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use arrayvec::ArrayString;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct EnchantmentKind(ArrayString<16>);

impl<'a, T: Into<&'a str>> From<T> for EnchantmentKind {
    fn from(value: T) -> Self {
        let id = value.into();

        let id = ArrayString::from(id)
            .expect("EnchantmentKind can't have an ID longer than 16 characters");

        Self(id)
    }
}

impl Display for EnchantmentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
