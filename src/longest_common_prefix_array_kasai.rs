pub fn lcp_array(
    a: &[usize],
    sa: &[usize],
) -> Vec<usize> {
    let n = a.len();

    assert!(n > 0 && sa.len() == n);

    let mut rank = vec![0; n];

    for (r, &i) in sa.iter().enumerate() {
        rank[i] = r;
    }

    let mut lcp = vec![0; n - 1];

    let mut h = 0;

    for i in 0..n {
        if h != 0 {
            h -= 1;
        }

        let r = rank[i];

        if r == n - 1 {
            continue;
        }

        let j = sa[r + 1];

        while i.max(j) + h < n && a[i + h] == a[j + h] {
            h += 1;
        }

        lcp[r] = h;
    }

    lcp
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
