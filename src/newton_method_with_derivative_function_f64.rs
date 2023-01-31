//! Newton's Method
//! f(x) must be differentiabl and f'(x) != 0
//! example
//! f(x) = x^2 - 10.
//! f'(x) = 2x
//! x = 3.16227...
//! TODO: use generic instead of f64 accepting int, big-rational, etc.

/// tol:  absolute tolerance
/// rtol: relative tolerance
#[derive(Default)]

pub struct Options {
    pub tol: Option<f64>,
    pub rtol: Option<f64>,
    pub max_iter: Option<u8>,
}

impl Options {
    pub fn new(
        tol: Option<f64>,
        rtol: Option<f64>,
        max_iter: Option<u8>,
    ) -> Self {
        Self { tol, rtol, max_iter }
    }
}

/// f := f(x)
/// fp := f'(x)
/// x0 := initial guess
/// Err(x) if not terminated in max iterations.

pub fn root<F, D>(
    f: &F,
    fp: &D,
    x0: f64,
    opts: Option<Options>,
) -> Result<f64, f64>
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    const MAX_ITER: u8 = 1 << 6;

    let opts = opts.unwrap_or_default();

    let tol = opts.tol.unwrap_or_default();

    let rtol = opts.rtol.unwrap_or_default();

    let max_iter = opts.max_iter.unwrap_or(MAX_ITER);

    assert!(0. <= tol);

    assert!(0. <= rtol && rtol < 1.);

    let mut x = x0;

    for _ in 0..max_iter {
        let d = f(x) / fp(x);

        x -= d;

        if d.abs() < tol && (d / x).abs() < rtol {
            return Ok(x);
        }
    }

    Err(x)
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::*;

        let n = 51628730198202384.;

        let f = |x: f64| x * x - n;

        let fp = |x: f64| 2. * x;

        let x = root(&f, &fp, 100_000_000., None);

        let x = x.unwrap_err();

        assert_eq!(x as u64, 227219563); // 227219563.854...
    }
}

// TODO:
/// newton's method for 2D function

pub fn newton2d() {}
