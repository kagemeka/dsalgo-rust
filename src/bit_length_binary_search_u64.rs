/// O(\log\log{N}})

pub fn binary_search(mut n: u64) -> u8 {
    let mut l = 0;

    for i in (0..6).rev() {
        let d = 1 << i;

        if n >> d > 0 {
            n >>= d;

            l += d;
        }
    }

    if n == 1 {
        l += 1;

        n -= 1;
    }

    debug_assert_eq!(n, 0);

    l
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
