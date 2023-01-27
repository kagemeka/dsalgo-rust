use crate::{
    const_modulus_trait::Modulus,
    static_modulus_trait::*,
};

// depends on impl_static_modulus_get_for_const_modulus
macro_rules! define_const_mod {
    ($name:ident, $uint:ty) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

        pub struct $name<const MOD: $uint>;

        impl<const MOD: $uint> Modulus for $name<MOD> {
            type T = $uint;

            const MOD: $uint = MOD;
        }
    };
}

define_const_mod!(ModU64, u64);

define_const_mod!(ModU32, u32);

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_const_mod() {
        type Mod = ModU32<1_000_000_007>;

        assert_eq!(Mod::get(), 1_000_000_007);
    }
}
