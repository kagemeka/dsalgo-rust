pub fn xor_gaussian_elimination(mut a: Vec<usize>) -> Vec<usize> {
    let n = a.len();

    let mut i = 0;

    for k in (0..64).rev() {
        let mut j = i;

        while j < n {
            if a[j] >> k & 1 == 1 {
                break;
            }

            j += 1;
        }

        if j == n {
            continue;
        }

        a.swap(i, j);

        let v = a[i];

        for (j, x) in a.iter_mut().enumerate() {
            if j == i {
                continue;
            }

            if *x >> k & 1 == 1 {
                *x ^= v;
            }
        }

        i += 1;
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(vec![3, 6, 5], vec![5, 3, 0])];

        for (a, ans) in cases {
            assert_eq!(xor_gaussian_elimination(a), ans);
        }
    }
}
