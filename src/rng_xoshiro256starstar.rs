use crate::{
    bitops::rot_l,
    rng_xoshiro256_core::xorshiro256_core,
};

pub struct Xoshiro256StarStar {
    seeds: [u64; 4],
}

impl Xoshiro256StarStar {
    pub fn new(seeds: [u64; 4]) -> Self { Self { seeds } }

    pub fn next(&mut self) -> u64 {
        let res = rot_l(self.seeds[1] * 5, 7) * 9;

        self.seeds = xorshiro256_core(self.seeds);

        res
    }
}

impl Default for Xoshiro256StarStar {
    fn default() -> Self { Self::new([1; 4]) }
}

// TODO:
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
