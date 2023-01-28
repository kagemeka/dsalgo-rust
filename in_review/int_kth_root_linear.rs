/// for k >= 10, faster than binary search.

pub fn int_kth_root_linear(
    n: u64,
    k: u8,
) -> u64 {
    assert!(k > 0);

    if k == 1 || n <= 1 {
        return n;
    }

    let mut x: u64 = 0;

    while let Some(y) = x.checked_pow(k as u32) {
        if y > n {
            break;
        }

        x += 1;
    }

    x - 1
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;
        use crate::test_int_kth_root::{
            test_int_kth_root,
            CASES_LINEAR,
        };

        test_int_kth_root(&int_kth_root_linear, CASES_LINEAR);
    }
}
