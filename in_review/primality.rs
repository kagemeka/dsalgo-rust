//! number theory algorithms on primality

pub mod test {

    //! primality test algorithms (not unit tests)

    use crate::rng_static_xorshift64::static_xorshift64;

    /// pre-generate random bases for some tests.

    pub fn rng_bases(k: usize) -> Vec<u64> {
        (0..k).map(|_| static_xorshift64()).collect()
    }

    pub(crate) mod test_cases {

        #[allow(dead_code)]

        pub(crate) const P: &'static [u64] = &[2, 998_244_353, 1_000_000_007];

        #[allow(dead_code)]

        pub(crate) const NP: &'static [u64] = &[0, 1, 561, 512_461];
    }

    pub(crate) fn trivial_primality(n: u64) -> Option<bool> {
        if n == 2 {
            return Some(true);
        }

        if n < 2 || n & 1 == 0 {
            return Some(false);
        }

        None
    }

    /// n is composite but b^(n-1) = 1 (mod n) for all b such gcd(b, n) = 1

    pub const CARMICHAEL_NUMS: &'static [u64] = &[
        561, 1105, 1729, 2465, 2821, 6601, 8911, 10585, 15841, 29341, 41041,
        46657, 52633, 62745, 63973, 75361, 101101, 115921, 126217, 162401,
        172081, 188461, 252601, 278545, 294409, 314821, 334153, 340561, 399001,
        410041, 449065, 488881, 512461,
    ];

    pub mod mr {

        //! Miller Rabin's Test

        pub mod bases {

            //! deterministic Miller Rabin bases

            pub const B32: [u32; 3] = [2, 7, 61];

            pub const B64_12: [u64; 12] =
                [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

            pub const B64_7: [u64; 7] =
                [2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022];
        }

        pub fn is_p(n: u64) -> bool {
            use crate::{
                montgomery_modular_multiplication_64::*,
                power::pow_semigroup,
            };

            if let Some(bl) = super::trivial_primality(n) {
                return bl;
            }

            let s = (n - 1).trailing_zeros();

            let d = (n - 1) >> s;

            // n - 1 = 2^s*d
            let m = MontgomeryMultiplication64::new(n);

            let mul = |x, y| m.mul(x, y);

            let is_c = |b| -> bool {
                let mut x = pow_semigroup(&mul, b, d);

                if x == 1 {
                    return false;
                }

                for _ in 0..s {
                    if x == n - 1 {
                        return false;
                    }

                    x = mul(x, x);
                }

                true
            };

            bases::B64_7
                .iter()
                .map(|&b| b % n)
                .filter(|&b| 2 <= b && b < n - 1)
                .all(|b| !is_c(b))
        }

        #[test]

        fn test_mr() {
            use super::test_cases::*;

            for x in P.into_iter() {
                assert!(is_p(*x));
            }

            for x in NP.into_iter() {
                assert!(!is_p(*x));
            }
        }
    }

    pub mod solovay_strassen {

        //! solovay strassen's test

        use crate::{
            euler_criterion::try_euler_criterion,
            jacobi_symbol::jacobi_symbol,
        };

        pub fn is_p(
            b: &[u64],
            n: u64,
        ) -> bool {
            if let Some(bl) = super::trivial_primality(n) {
                return bl;
            }

            // compare jcobi symbol and euler's criterion.
            let is_c = |b| -> bool {
                let j = jacobi_symbol(n, b);

                if j == 0 {
                    return true;
                }

                if let Ok(e) = try_euler_criterion(n, b) {
                    let j = if j == 1 { 1 } else { n - 1 };

                    e != j
                } else {
                    true
                }
            };

            b.iter()
                .map(|&b| b % n)
                .filter(|&b| 2 <= b && b < n)
                .all(|b| !is_c(b))
        }

        #[test]

        fn test() {
            use super::{
                rng_bases,
                test_cases::*,
            };

            let bases = rng_bases(20);

            for x in P.into_iter() {
                assert!(is_p(&bases, *x));
            }

            for x in NP.into_iter() {
                assert!(!is_p(&bases, *x));
            }
        }
    }

    pub mod miller_rabin_solovay_strassen {

        //! Miller Rabin - Solovay Strassen's Test
    }

    pub mod fermat {

        //! Fermat's Test

        use crate::{
            greatest_common_divisor_euclidean::gcd,
            montgomery_modular_multiplication_64::MontgomeryMultiplication64,
            power::pow_semigroup,
        };

        pub fn is_p(
            b: &[u64],
            n: u64,
        ) -> bool {
            if let Some(bl) = super::trivial_primality(n) {
                return bl;
            }

            let m = MontgomeryMultiplication64::new(n);

            let mul = |x, y| m.mul(x, y);

            b.iter()
                .map(|&b| b % n)
                .filter(|&b| 2 <= b && b < n - 1)
                .all(|b| gcd(b, n) == 1 && pow_semigroup(&mul, b, n - 1) == 1)
        }

        #[test]

        fn test() {
            use super::{
                rng_bases,
                test_cases::*,
            };

            let bases = rng_bases(30);

            for x in P.into_iter() {
                assert!(is_p(&bases, *x));
            }

            for x in NP.into_iter() {
                assert!(!is_p(&bases, *x));
            }

            for x in super::CARMICHAEL_NUMS.iter() {
                assert!(!is_p(&bases, *x));
            }
        }
    }

    pub mod adleman_pomerance_rumely {}

    pub mod aks {

        //! Agrawal, Kayal, Sexena test

        // TODO:
    }

    pub mod atkin_morain_elliptic_curve {}

    pub mod baillie_psw {}

    pub mod elliptic_curve {}

    pub mod frobenius {}

    pub mod goldwasser_kilian {}

    pub mod lucas {}

    pub mod lucas_lehmer {}

    pub mod lucas_lehmer_reisel {}

    pub mod miller {}

    pub mod pepin {}

    pub mod pocklington {}

    pub mod proth_theorem {}

    pub mod trial_division {
        // TODO
        // use prime factorize trial division.
        // kind of factors is just one.
    }

    pub mod wilson_theorem {}
}
