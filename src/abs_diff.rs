/// if >= v1.60.0, a.abs_diff(b) is available.
/// https://doc.rust-lang.org/std/primitive.u64.html#method.abs_diff
use std::ops::Sub;
pub fn abs_diff<T: Sub<Output = T> + Ord>(a: T, b: T) -> T {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

#[test]
fn test() {
    assert_eq!(abs_diff(1u64, 2u64), 1);
}
