pub fn pow_semigroup<F, X>(
    f: F,
    mut x: X,
    mut exp: u64,
) -> X
where
    F: Fn(X, X) -> X,
    X: Clone,
{
    assert!(exp > 0);

    let mut y = x.clone();

    exp -= 1;

    while exp > 0 {
        if exp & 1 == 1 {
            y = f(y, x.clone());
        }

        x = f(x.clone(), x);

        exp >>= 1;
    }

    y
}
