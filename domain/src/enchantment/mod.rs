mod combine;
mod kind;

use std::fmt::Display;
use std::fmt::Formatter;

pub use combine::*;
pub use kind::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct Enchantment {
    pub kind: EnchantmentKind,
    pub level: u8,
}

impl Display for Enchantment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Enchantment { kind, level } = self;
        write!(f, "{kind} {level}")
    }
}
