pub fn xorshift64(seed: u64) -> u64 {
    let mut x = seed;

    x ^= x << 13;

    x ^= x >> 7;

    x ^= x << 17;

    x
}

pub fn xorshift64_fast(seed: u64) -> u64 {
    let mut x = seed;

    x ^= x << 7;

    x ^ (x >> 9)
}

pub struct Xorshift64(u64);

impl Xorshift64 {
    pub fn next(&mut self) -> u64 {
        self.0 = xorshift64(self.0);

        self.0
    }
}

impl Default for Xorshift64 {
    fn default() -> Self { Xorshift64(88172645463325252) }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;

        const ANS: [u64; 3] =
            [8748534153485358512, 3040900993826735515, 3453997556048239312];

        let mut rng = Xorshift64::default();

        for i in 0..3 {
            assert_eq!(rng.next(), ANS[i]);
        }
    }
}
