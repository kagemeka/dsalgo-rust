use crate::const_modulus_trait::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct Mod1_000_000_007;

impl Modulus for Mod1_000_000_007 {
    type T = i64;

    const MOD: i64 = 1_000_000_007;
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct Mod998_244_353;

impl Modulus for Mod998_244_353 {
    type T = i64;

    const MOD: i64 = 998_244_353;
}
