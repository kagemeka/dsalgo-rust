/// if n >= 1 and k >= 64, answer is always 1 under 64bit.

pub fn int_kth_root_binary_search(
    n: u64,
    k: u8,
) -> u64 {
    assert!(k > 0);

    if k == 1 || n <= 1 {
        return n;
    }

    if k >= 64 {
        return 1;
    }

    let mut lo = 0;

    let mut hi = n;

    while hi - lo > 1 {
        let x = (lo + hi) >> 1;

        if let Some(y) = x.checked_pow(k as u32) {
            if y <= n {
                lo = x;
            } else {
                hi = x;
            }
        } else {
            hi = x;
        }
    }

    lo
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;
        use crate::test_int_kth_root::{
            test_int_kth_root,
            CASES_ITERATIVE,
            CASES_LINEAR,
        };

        test_int_kth_root(&int_kth_root_binary_search, CASES_ITERATIVE);

        test_int_kth_root(&int_kth_root_binary_search, CASES_LINEAR);
    }
}
