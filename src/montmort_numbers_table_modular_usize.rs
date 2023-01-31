pub fn montmort_numbers(
    m: usize,
    size: usize,
) -> Vec<usize> {
    let mut a = vec![0; size];

    a[0] = 1;

    for i in 2..size {
        a[i] = (a[i - 2] + a[i - 1]) * (i - 1) % m;
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::const_montmort_numbers_usize::MONTMORT_NUMBERS;

        let n = MONTMORT_NUMBERS.len();

        const MOD: usize = 100;

        let res = montmort_numbers(MOD, n);

        assert_eq!(
            res,
            MONTMORT_NUMBERS.iter().map(|x| x % MOD).collect::<Vec<_>>()
        );
    }
}
