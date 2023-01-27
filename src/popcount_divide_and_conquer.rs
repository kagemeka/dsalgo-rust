/// O(\log\log{N})

pub fn popcount(mut n: u64) -> u8 {
    const MASKS: [u64; 6] = [
        0x5555555555555555,
        0x3333333333333333,
        0x0f0f0f0f0f0f0f0f,
        0x00ff00ff00ff00ff,
        0x0000ffff0000ffff,
        0x00000000ffffffff,
    ];

    for (i, m) in MASKS.into_iter().enumerate() {
        n = (n & m) + (n >> (1 << i) & m);
    }

    n as u8
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = [(0b001010101, 4), (0b11111, 5), (0b00000, 0)];

        for (n, ans) in cases {
            assert_eq!(popcount(n), ans);
        }
    }
}
