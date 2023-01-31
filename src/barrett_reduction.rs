use crate::{
    bit_length_with_count_leading_zeros_u64::bit_length,
    karatsuba_mul_quotient_pow_2_power_of_2_128::*,
};

pub struct BarrettReduction {
    n: u128,
    k: u8,
    m: u128,
}

impl BarrettReduction {
    pub fn new(modulus: u64) -> Self {
        assert!(0 < modulus && modulus >> 63 == 0);

        let n = modulus as u128;

        let k = bit_length(modulus) << 1;

        let m = (1u128 << k) / n;

        Self { n, k, m }
    }

    // TODO: any faster algorithm than karatsuba to avoid overflow?
    pub fn reduce(
        &self,
        mut x: u128,
    ) -> u64 {
        assert!(x < self.n.pow(2));

        let q = karatsuba_mul_quotient_pow_2_power_of_2(self.k >> 1, x, self.m);

        x -= q * self.n;

        if x >= self.n {
            x -= self.n;
        }

        debug_assert!(x < self.n);

        x as u64
    }
}

pub struct BarrettReduction32 {
    n: u128,
    m: u128,
}

impl BarrettReduction32 {
    pub fn new(modulus: u32) -> Self {
        let n = modulus as u128;

        let m = (1u128 << 64) / n as u128;

        Self { n, m }
    }

    pub fn reduce(
        &self,
        x: u64,
    ) -> u32 {
        let mut x = x as u128;

        assert!(x < self.n.pow(2));

        let q = (x as u128 * self.m) >> 64;

        x -= q * self.n;

        if x >= self.n {
            x -= self.n;
        }

        debug_assert!(x < self.n);

        x as u32
    }
}

pub struct BarrettReduction64 {
    n: u128,
    m0: u128,
    m1: u128,
}

impl BarrettReduction64 {
    const MASK: u128 = (1u128 << 63) - 1;

    pub fn new(modulus: u64) -> Self {
        let n = modulus as u128;

        assert!(n >> 63 == 0);

        let m = (1u128 << 126) / n;

        let (m1, m0) = (m >> 63, m & Self::MASK);

        Self { n, m0, m1 }
    }

    pub fn reduce(
        &self,
        mut x: u128,
    ) -> u64 {
        assert!(x < self.n.pow(2));

        let (x1, x0) = (x >> 63, x & Self::MASK);

        let t2 = x1 * self.m1;

        let t0 = x0 * self.m0;

        let t1 = x1 * self.m0 + x0 * self.m1;

        let q = t2 + ((t1 + (t0 >> 63)) >> 63);

        x -= q * self.n;

        if x >= self.n {
            x -= self.n;
        }

        debug_assert!(x < self.n);

        x as u64
    }
}

// TODO:
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
