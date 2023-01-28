use crate::{
    const_modulus_trait::Modulus,
    static_modulus_trait::Get,
};

macro_rules! define_const_mod_old {
    ($name:ident, $uint:ty, $value:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

        pub struct $name;

        impl Modulus for $name {
            type T = $uint;

            const MOD: Self::T = $value;
        }
    };
}

define_const_mod_old!(Mod998_244_353, u32, 998_244_353);

define_const_mod_old!(Mod1_000_000_007, u32, 1_000_000_007);

define_const_mod_old!(Mod1_000_000_007I64, i64, 1_000_000_007);

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        type Mod = Mod1_000_000_007;

        assert_eq!(Mod::get(), 1_000_000_007);
    }
}
