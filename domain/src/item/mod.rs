mod builder;
mod kind;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub use kind::*;

use crate::enchantment::Enchantment;
use crate::enchantment::EnchantmentKind;

pub type PreviousLevel = u8;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Item {
    kind: ItemKind,
    enchantments: HashMap<EnchantmentKind, u8>,
}

impl Item {
    pub fn new(kind: impl Into<ItemKind>) -> Self {
        Self {
            kind: kind.into(),
            enchantments: HashMap::new(),
        }
    }

    pub fn kind(&self) -> ItemKind {
        self.kind
    }

    pub fn enchantments(&self) -> impl Iterator<Item = Enchantment> {
        self.enchantments
            .iter()
            .map(|(kind, level)| Enchantment::new(*kind, *level))
    }

    pub fn enchant(&mut self, enchantment: Enchantment) -> Option<PreviousLevel> {
        self.enchantments
            .insert(enchantment.kind(), enchantment.level())
    }

    pub fn unenchant(&mut self, kind: EnchantmentKind) -> Option<PreviousLevel> {
        self.enchantments.remove(&kind)
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}[", self.kind())?;

        for (index, enchantment) in self.enchantments().enumerate() {
            if index != 0 {
                write!(f, ",")?;
            }

            enchantment.fmt(f)?;
        }

        write!(f, "]")
    }
}
