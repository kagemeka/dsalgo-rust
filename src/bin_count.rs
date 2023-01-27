pub fn bin_count(a: &[usize]) -> Vec<usize> {
    let mut cnt = vec![0; *a.iter().max().unwrap() + 1];

    for &x in a.iter() {
        cnt[x] += 1;
    }

    cnt
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![1, 3, 4, 3];

        assert_eq!(bin_count(&a), vec![0, 1, 0, 2, 1]);
    }
}
