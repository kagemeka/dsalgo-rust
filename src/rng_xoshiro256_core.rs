use crate::bitops::rot_l;

pub(crate) fn xorshiro256_core(seeds: [u64; 4]) -> [u64; 4] {
    let [mut x, mut y, mut z, mut w] = seeds;

    let t = y << 17;

    z ^= x;

    w ^= y;

    y ^= z;

    x ^= w;

    z ^= t;

    w = rot_l(z, 45);

    [x, y, z, w]
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
