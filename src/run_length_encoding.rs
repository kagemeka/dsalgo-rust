pub fn rle<T>(a: &[T]) -> Vec<(T, usize)>
where
    T: PartialEq + Clone,
{
    let n = a.len();

    let mut res = vec![];

    if n == 0 {
        return res;
    }

    let mut cnt = 1;

    for i in 0..n - 1 {
        if a[i] == a[i + 1] {
            cnt += 1;
        } else {
            res.push((a[i].clone(), cnt));

            cnt = 1;
        }
    }

    res.push((a[n - 1].clone(), cnt));

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let s = "abbbbaaac".chars().collect::<Vec<_>>();

        assert_eq!(rle(&s), [('a', 1), ('b', 4), ('a', 3), ('c', 1)]);
    }
}
