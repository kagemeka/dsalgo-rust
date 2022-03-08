fn is_trivial_composite(n: usize) -> bool { n > 2 && n & 1 == 0 }

fn is_composite<M: crate::modular::Modulus + Copy>(
    n: usize,
    base: crate::modular::Modular<M>,
) -> bool {
    if is_trivial_composite(n) {
        return true;
    }
    let (mut s, mut d) = (0, n - 1);
    // n - 1 = 2^s*d
    while d & 1 == 0 {
        s += 1;
        d >>= 1;
    }
    let mut x = base.pow(d);
    for _ in 0..s {
        let y = x * x;
        if y.value() == 1 {
            return x.value() != 1 && x.value() != n - 1;
        }
        x = y;
    }
    true
}

fn miller_rabin_fixed_bases(n: usize, bases: &[usize]) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    #[derive(Clone, Copy)]
    struct Mod;
    use crate::{
        modular::{Modular, Modulus},
        modular_dynamic::{DynamicModulus, DynamicModulusCore},
    };
    impl DynamicModulus for Mod {
        fn core() -> &'static DynamicModulusCore {
            static MODULUS: DynamicModulusCore = DynamicModulusCore::new(1);
            &MODULUS
        }
    }
    impl Modulus for Mod {
        fn value() -> usize { Self::core().get_value() }
    }
    Mod::set(n);
    type Mint = Modular<Mod>;
    for base in bases.iter().map(|x| Mint::new(*x)) {
        if is_composite(n, base) {
            return false;
        }
    }
    true
}

pub fn miller_rabin_test(n: usize, check_times: usize) -> bool {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let mut bases = (0..check_times)
        .map(|_| rng.gen_range(2..n))
        .collect::<Vec<_>>();
    bases = crate::vector_util::unique(&bases);
    miller_rabin_fixed_bases(n, &bases)
}

pub fn miller_rabin_test_32(n: usize) -> bool {
    static BASES: [usize; 3] = [2, 7, 61];
    miller_rabin_fixed_bases(n, &BASES)
}

pub fn miller_rabin_test_64(n: usize) -> bool {
    static BASES: [usize; 12] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
    ];
    miller_rabin_fixed_bases(n, &BASES)
}

pub fn miller_rabin_test_64_v2(n: usize) -> bool {
    static BASES: [usize; 7] = [
        2, 325, 9375, 28178, 450775, 9780504, 1795265022,
    ];
    miller_rabin_fixed_bases(n, &BASES)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::miller_rabin_test_32(998_244_353), true);
        assert_eq!(super::miller_rabin_test_32(1_000_000_007), true);
        assert_eq!(super::miller_rabin_test_32(1_000_000_007), true);
        assert_eq!(super::miller_rabin_test_32(561), false);
        assert_eq!(super::miller_rabin_test_32(512461), false);
        assert_eq!(super::miller_rabin_test(512461, 20), false);
    }
}
