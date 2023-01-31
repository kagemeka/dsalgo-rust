use crate::dynamic_modulus_trait::*;

/// T is gonna be u64 or u32
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct DynMod<T>(T);

impl<T> DynMod<T> {
    pub fn new(value: T) -> Self { Self(value) }
}

impl<T: Copy> Get for DynMod<T> {
    type T = T;

    fn get(&self) -> Self::T { self.0 }
}

impl<T> Set for DynMod<T> {
    type T = T;

    fn set(
        &mut self,
        value: Self::T,
    ) {
        self.0 = value
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_dyn_mod() {
        type Mod = DynMod<u32>;

        let mut m = Mod::new(998_244_353);

        assert_eq!(m.get(), 998_244_353);

        m.set(1_000_000_007);

        assert_eq!(m.get(), 1_000_000_007);
    }
}
