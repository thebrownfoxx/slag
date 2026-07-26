mod bedrock;
mod java;
mod unbound;

pub use bedrock::*;
pub use java::*;
pub use unbound::*;

use super::EnchantmentKind;

pub trait CombineEnchantments {
    fn combine(&self, kind: EnchantmentKind, target_level: u8, sacrifice_level: u8) -> Option<u8>;
}
