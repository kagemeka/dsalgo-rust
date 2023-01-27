pub fn popcount(mut n: u64) -> u8 {
    const M0: u64 = 0x5555555555555555; // 0b0101...
    const M1: u64 = 0x3333333333333333; // 0b0011...
    const M2: u64 = 0x0f0f0f0f0f0f0f0f; // 0b00001111...
    n -= (n >> 1) & M0;

    n = (n & M1) + ((n >> 2) & M1);

    n = (n + (n >> 4)) & M2;

    n += n >> 8;

    n += n >> 16;

    n += n >> 32;

    return (n & 0x7f) as u8;
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
