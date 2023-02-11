pub fn lis(a: &[i64]) -> Vec<i64> {
    let n = a.len();

    const INF: i64 = std::i64::MAX;

    let mut f = vec![INF; n];

    for &x in a.iter() {
        for v in f.iter_mut() {
            if *v < x {
                continue;
            }

            *v = x;

            break;
        }
    }

    for (i, v) in f.iter().enumerate() {
        if *v != INF {
            continue;
        }

        return f[..i].to_vec();
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let arr = [4, 2, 8, 5, 6, 6];

        assert_eq!(lis(&arr), vec![2, 5, 6]);
    }
}
