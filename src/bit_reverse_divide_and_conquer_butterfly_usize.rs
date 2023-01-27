pub fn bit_reverse(mut n: usize) -> usize {
    const MASKS: [usize; 6] = [
        0xaaaaaaaaaaaaaaaa,
        0xcccccccccccccccc,
        0xf0f0f0f0f0f0f0f0,
        0xff00ff00ff00ff00,
        0xffff0000ffff0000,
        0xffffffff00000000,
    ];

    for (i, &m) in MASKS.iter().enumerate() {
        let d = 1 << i;

        n = ((n & m) >> d) | ((n & !m) << d);
    }

    n
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(bit_reverse(0xffffffff0000f000), 0x000f0000ffffffff);
    }
}
