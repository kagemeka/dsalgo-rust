#[allow(dead_code)]

pub(crate) const CASES_ITERATIVE: &'static [(u64, u8, u64)] = &[
    (100, 1, 100),
    (100, 2, 10),
    (100, 3, 4),
    (100, 4, 3),
    (100, 5, 2),
    (100, 6, 2),
    (100, 7, 1),
    (215, 3, 5),
    (216, 3, 6),
    (217, 3, 6),
    (9999999999, 10, 9),
    (10000000000, 10, 10),
    (10000000001, 10, 10),
    (18446744073709551615, 1, 18446744073709551615),
    (18446744073709551615, 2, 4294967295),
    (18446744073709551615, 63, 2),
    (18446744073709551615, 64, 1),
];

#[allow(dead_code)]

pub(crate) const CASES_LINEAR: &'static [(u64, u8, u64)] = &[
    (std::u64::MAX, 10, 84),
    (std::u64::MAX, 11, 56),
    (std::u64::MAX, 12, 40),
    (std::u64::MAX, 13, 30),
    (std::u64::MAX, 14, 23),
    (std::u64::MAX, 15, 19),
    (std::u64::MAX, 16, 15),
    (std::u64::MAX, 17, 13),
    (std::u64::MAX, 18, 11),
    (std::u64::MAX, 19, 10),
    (std::u64::MAX, 20, 9),
];

#[allow(dead_code)]

pub(crate) fn test_int_kth_root<F>(
    int_kth_root: &F,
    cases: &[(u64, u8, u64)],
) where
    F: Fn(u64, u8) -> u64,
{
    for &(n, k, ans) in cases {
        assert_eq!(int_kth_root(n, k), ans);
    }
}
