pub fn sa_is(mut a: Vec<usize>) -> Vec<usize> {
    fn preprocess(
        a: &[usize]
    ) -> (Vec<bool>, Vec<bool>, Vec<usize>, Vec<usize>, Vec<usize>) {
        let n = a.len();

        let m = a.iter().max().unwrap() + 1;

        let mut is_s = vec![true; n];

        let mut is_lms = vec![false; n];

        let mut lms = Vec::with_capacity(n);

        for i in (1..n).rev() {
            is_s[i - 1] =
                if a[i - 1] == a[i] { is_s[i] } else { a[i - 1] < a[i] };

            is_lms[i] = !is_s[i - 1] && is_s[i];

            if is_lms[i] {
                lms.push(i);
            }
        }

        lms.reverse();

        let mut arg_l = vec![0; m];

        let mut arg_r = vec![0; m];

        for &x in a.iter() {
            arg_r[x] += 1;

            if x < m - 1 {
                arg_l[x + 1] += 1;
            }
        }

        for i in 0..m - 1 {
            arg_r[i + 1] += arg_r[i];

            arg_l[i + 1] += arg_l[i];
        }

        (is_s, is_lms, lms, arg_l, arg_r)
    }

    fn induced_sort(
        a: &[usize],
        is_s: &[bool],
        lms: &[usize],
        arg_l: &[usize],
        arg_r: &[usize],
    ) -> Vec<usize> {
        let n = a.len();

        let mut sa = vec![n; n];

        let mut arg = arg_r.to_vec();

        for &i in lms.iter().rev() {
            arg[a[i]] -= 1;

            sa[arg[a[i]]] = i;
        }

        let mut arg = arg_l.to_vec();

        for i in 0..n {
            if sa[i] == n || sa[i] == 0 {
                continue;
            }

            let i = sa[i] - 1;

            if !is_s[i] {
                sa[arg[a[i]]] = i;

                arg[a[i]] += 1;
            }
        }

        let mut arg = arg_r.to_vec();

        for i in (0..n).rev() {
            if sa[i] == n || sa[i] == 0 {
                continue;
            }

            let i = sa[i] - 1;

            if is_s[i] {
                arg[a[i]] -= 1;

                sa[arg[a[i]]] = i;
            }
        }

        sa
    }

    fn compute_lms_rank(
        a: &[usize],
        is_s: &[bool],
        is_lms: &[bool],
        lms: &[usize],
        arg_l: &[usize],
        arg_r: &[usize],
    ) -> Vec<usize> {
        let n = a.len();

        let l = lms.len();

        let lms_idx = induced_sort(a, is_s, lms, arg_l, arg_r)
            .into_iter()
            .filter(|&i| is_lms[i])
            .collect::<Vec<_>>();

        let mut rank = vec![n; n];

        let mut r = 0;

        rank[n - 1] = r;

        for i in 0..l - 1 {
            let j = lms_idx[i];

            let k = lms_idx[i + 1];

            for d in 0..n {
                if a[j + d] != a[k + d] {
                    r += 1;

                    break;
                }

                if d > 0 && is_lms[j + d] {
                    if !is_lms[k + d] {
                        r += 1;
                    }

                    break;
                }
            }

            rank[k] = r;
        }

        rank.into_iter().filter(|&x| x != n).collect()
    }

    let m = *a.iter().min().unwrap();

    for x in a.iter_mut() {
        *x = *x - m + 1;
    }

    a.push(0);

    let mut st = vec![];

    let mut lms_order;

    loop {
        let n = a.len();

        let (is_s, is_lms, lms, arg_l, arg_r) = preprocess(&a);

        let rank = compute_lms_rank(&a, &is_s, &is_lms, &lms, &arg_l, &arg_r);

        let l = lms.len();

        st.push((a, is_s, lms, arg_l, arg_r));

        a = rank;

        if *a.iter().max().unwrap() < l - 1 {
            continue;
        }

        lms_order = vec![n; l];

        for i in 0..l {
            lms_order[a[i]] = i;
        }

        break;
    }

    while let Some((a, is_s, mut lms, arg_l, arg_r)) = st.pop() {
        lms = lms_order.into_iter().map(|i| lms[i]).collect();

        lms_order = induced_sort(&a, &is_s, &lms, &arg_l, &arg_r);
    }

    lms_order[1..].to_vec()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn suffix_array() {
        let cases = vec![
            (
                vec![1, 1, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 2, 2, 0, 0],
                vec![15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4],
            ),
            (
                vec![1, 0, 3, 3, 0, 3, 3, 0, 2, 2, 0],
                vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2],
            ),
        ];

        for (s, ans) in cases {
            assert_eq!(sa_is(s), ans);
        }
    }
}
