#[macro_export]
macro_rules! item {
    ($kind:expr) => {
        $crate::item::Item::new($kind)
    };
    ($kind:expr, $($enchantment:expr),+ $(,)?) => {
        {
            let mut item = $crate::item::Item::new($kind);
            $(
                item.enchant($enchantment);
            )+
            item
        }
    };
}
