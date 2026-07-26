use std::cmp::min;

use super::CombineEnchantments;
use super::EnchantmentKind;
use super::combine_enchantment_levels;

#[derive(Debug)]
pub struct JavaEnchantmentCombiner<Max>
where
    Max: Fn(EnchantmentKind) -> u8,
{
    max_level: Max,
}

impl<Max> JavaEnchantmentCombiner<Max>
where
    Max: Fn(EnchantmentKind) -> u8,
{
    pub fn new(max_level: Max) -> Self {
        Self { max_level }
    }
}

impl<Max> CombineEnchantments for JavaEnchantmentCombiner<Max>
where
    Max: Fn(EnchantmentKind) -> u8,
{
    fn combine(&self, kind: EnchantmentKind, target_level: u8, sacrifice_level: u8) -> Option<u8> {
        let max_level = (self.max_level)(kind);
        Some(combine(target_level, sacrifice_level, max_level))
    }
}

fn combine(target_level: u8, sacrifice_level: u8, max_level: u8) -> u8 {
    let combined = combine_enchantment_levels(target_level, sacrifice_level);
    min(combined, max_level)
}
