pub fn pow_semigroup_recurse<F, X>(
    f: F,
    x: X,
    exp: u64,
) -> X
where
    F: Fn(X, X) -> X,
    X: Clone,
{
    assert!(exp > 0);

    if exp == 1 {
        return x;
    }

    let mut y = pow_semigroup_recurse(&f, x.clone(), exp >> 1);

    y = f(y.clone(), y);

    if exp & 1 == 1 {
        y = f(y, x);
    }

    y
}
