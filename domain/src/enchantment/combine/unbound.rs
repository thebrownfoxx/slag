use std::cmp::max;

use super::CombineEnchantments;
use super::EnchantmentKind;

#[derive(Debug)]
pub struct UnboundEnchantmentCombiner;

impl CombineEnchantments for UnboundEnchantmentCombiner {
    fn combine(&self, _: EnchantmentKind, target_level: u8, sacrifice_level: u8) -> Option<u8> {
        Some(combine_enchantment_levels(target_level, sacrifice_level))
    }
}

pub(super) fn combine_enchantment_levels(target_level: u8, sacrifice_level: u8) -> u8 {
    if target_level == sacrifice_level {
        return target_level + 1;
    }

    max(target_level, sacrifice_level)
}
