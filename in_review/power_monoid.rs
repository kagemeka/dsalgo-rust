use crate::power_semigroup::pow_semigroup;

pub fn pow_monoid<F, E, X>(
    f: F,
    e: &E,
    x: X,
    exp: u64,
) -> X
where
    F: Fn(X, X) -> X,
    E: Fn() -> X,
    X: Clone,
{
    if exp == 0 {
        e()
    } else {
        pow_semigroup(f, x, exp)
    }
}
