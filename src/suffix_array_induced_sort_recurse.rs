pub fn sa_is(mut a: Vec<usize>) -> Vec<usize> {
    let m = *a.iter().min().unwrap();

    for x in a.iter_mut() {
        *x = *x - m + 1;
    }

    a.push(0);

    let n = a.len();

    let m = a.iter().max().unwrap() + 1;

    let mut is_s = vec![true; n];

    let mut is_lms = vec![false; n];

    let mut lms = Vec::with_capacity(n);

    for i in (1..n).rev() {
        is_s[i - 1] = if a[i - 1] == a[i] { is_s[i] } else { a[i - 1] < a[i] };

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

    let induce = |lms: &Vec<usize>| -> Vec<usize> {
        let mut sa = vec![n; n];

        let mut arg = arg_r.clone();

        for &i in lms.iter().rev() {
            arg[a[i]] -= 1;

            sa[arg[a[i]]] = i;
        }

        let mut arg = arg_l.clone();

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

        let mut arg = arg_r.clone();

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
    };

    let l = lms.len();

    let lms_idx =
        induce(&lms).into_iter().filter(|&i| is_lms[i]).collect::<Vec<_>>();

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

    rank = rank.into_iter().filter(|&x| x != n).collect();

    let mut lms_order: Vec<usize> = Vec::new();

    if r == l - 1 {
        lms_order.resize(l, n);

        for i in 0..l {
            lms_order[rank[i]] = i;
        }
    } else {
        lms_order = sa_is(rank);
    }

    lms = lms_order.iter().map(|&i| lms[i]).collect();

    let sa = induce(&lms);

    sa[1..].to_vec()
}

pub fn from_str(s: &str) -> Vec<usize> {
    sa_is(s.as_bytes().iter().map(|&x| x as usize).collect())
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
