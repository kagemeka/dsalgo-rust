pub fn count_swap<T: Eq>(
    mut a: Vec<T>,
    target: &[T],
) -> usize {
    let mut cnt = 0;

    let n = a.len();

    assert_eq!(target.len(), n);

    for i in 0..n {
        for mut j in i..n {
            if a[j] != target[i] {
                continue;
            }

            while j > i {
                a.swap(j - 1, j);

                cnt += 1;

                j -= 1;
            }

            break;
        }
    }

    cnt
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(("catredo", "atcoder"), 8)];

        for ((a, t), ans) in cases {
            assert_eq!(count_swap(a.as_bytes().to_vec(), t.as_bytes()), ans);
        }
    }
}
