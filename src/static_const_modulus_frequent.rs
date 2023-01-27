use crate::static_modulus_trait::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct Mod1_000_000_007;

impl Get for Mod1_000_000_007 {
    type T = i64;

    fn get() -> Self::T { 1_000_000_007 }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]

pub struct Mod998_244_353;

impl Get for Mod998_244_353 {
    type T = i64;

    fn get() -> Self::T { 998_244_353 }
}
