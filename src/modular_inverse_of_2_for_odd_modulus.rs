pub fn mod_inv_of_2(m: u64) -> u64 {
    assert!(2 < m && m & 1 == 1);

    (m + 1) >> 1
}
