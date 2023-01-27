use crate::{
    power_monoid::pow_monoid,
    power_semigroup::pow_semigroup,
};

pub fn pow_group<F, E, Inv, X>(
    f: F,
    e: &E,
    inv: &Inv,
    x: X,
    exp: i64,
) -> X
where
    F: Fn(X, X) -> X,
    E: Fn() -> X,
    Inv: Fn(X) -> X,
    X: Clone,
{
    if exp >= 0 {
        pow_monoid(f, e, x, exp as u64)
    } else {
        pow_semigroup(f, inv(x), -exp as u64)
    }
}
