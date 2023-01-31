pub fn subset_sum_max(
    less_than: usize,
    a: &[usize],
) -> usize {
    let merge = |a: &[usize], x: usize| -> Vec<usize> {
        let mut i = 0;

        let n = a.len();

        let mut b = Vec::with_capacity(n << 1);

        for &y in a.iter() {
            while i < n && a[i] + x <= y {
                b.push(a[i] + x);

                i += 1;
            }

            b.push(y);
        }

        while i < n {
            b.push(a[i] + x);

            i += 1;
        }

        b
    };

    let mut b = vec![0];

    let mut c = vec![0];

    for &x in a {
        b = merge(&b, x);

        std::mem::swap(&mut b, &mut c);
    }

    let mut mx = 0;

    c.reverse();

    let mut i = 0;

    for x in b.into_iter().take_while(|&x| x < less_than) {
        while x + c[i] >= less_than {
            i += 1;
        }

        mx = mx.max(x + c[i]);
    }

    mx
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((vec![2, 3, 5, 7, 11], 18), 17),
            ((vec![1, 2, 7, 5, 8, 10], 101), 33),
            ((vec![101, 102, 103, 104, 105, 106], 101), 0),
            (
                (
                    vec![
                        6706927, 91566569, 89131517, 71069699, 75200339,
                        98298649, 92857057,
                    ],
                    273599681,
                ),
                273555143,
            ),
        ];

        for ((a, t), ans) in cases {
            assert_eq!(subset_sum_max(t, &a), ans);
        }
    }
}
