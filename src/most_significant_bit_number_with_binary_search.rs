/// O(\log\log{N})

pub fn msb_number(mut n: u64) -> u64 {
    const MASKS: [u64; 6] = [
        0xffffffff00000000,
        0xffff0000ffff0000,
        0xff00ff00ff00ff00,
        0xf0f0f0f0f0f0f0f0,
        0xcccccccccccccccc, // 0b1100...
        0xaaaaaaaaaaaaaaaa, // 0b1010...
    ];

    for m in MASKS.iter() {
        if n & m > 0 {
            n &= m;
        }
    }

    n
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (0b00000000, 0b00000000),
            (0b10000000, 0b10000000),
            (0b10001000, 0b10000000),
            (0b01010101, 0b01000000),
        ];

        for (n, ans) in cases {
            assert_eq!(msb_number(n), ans);
        }
    }
}
