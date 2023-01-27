pub fn karatsuba_mul_quotient_pow_2(base: u64, x: u128, y: u128) -> u128 {
    assert!(base <= 1 << 63);
    let base = base as u128;
    let base2 = base * base;
    assert!(x < base2 && y < base2);
    let (a1, a0) = (x / base, x % base);
    let (b1, b0) = (y / base, y % base);
    let c2 = a1 * b1;
    let c0 = a0 * b0;
    let c1 = (a1 + a0) * (b1 + b0) - c2 - c0;
    c2 + ((c1 + (c0 / base)) / base)
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
