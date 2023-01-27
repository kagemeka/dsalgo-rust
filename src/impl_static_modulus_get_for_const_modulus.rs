use crate::{
    const_modulus_trait::Modulus,
    static_modulus_trait::Get,
};

impl<M: Modulus> Get for M {
    type T = M::T;

    fn get() -> Self::T { Self::MOD }
}
