mod combine;
mod kind;

use std::fmt::Display;
use std::fmt::Formatter;

pub use combine::*;
pub use kind::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct Enchantment {
    kind: EnchantmentKind,
    level: u8,
}

impl Enchantment {
    pub fn new(kind: impl Into<EnchantmentKind>, level: u8) -> Self {
        Self {
            kind: kind.into(),
            level,
        }
    }

    pub fn kind(self) -> EnchantmentKind {
        self.kind
    }

    pub fn level(self) -> u8 {
        self.level
    }
}

impl Display for Enchantment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Enchantment { kind, level } = self;
        write!(f, "{kind} {level}")
    }
}
