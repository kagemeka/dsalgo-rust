pub fn sliding_window_maximum<T: Ord + Clone>(
    a: &[T],
    k: usize,
) -> Vec<T> {
    let n = a.len();

    assert!(k <= n);

    let mut que = std::collections::VecDeque::new();

    let mut res = Vec::with_capacity(n - k + 1);

    let mut f = |i: usize| -> usize {
        if !que.is_empty() && que.front().unwrap() + k == i {
            que.pop_front();
        }

        while !que.is_empty() && a[*que.back().unwrap()] <= a[i] {
            que.pop_back();
        }

        que.push_back(i);

        *que.front().unwrap()
    };

    for i in 0..n {
        let j = f(i);

        if i + 1 >= k {
            res.push(a[j].clone());
        }
    }

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]),
            ((vec![1], 1), vec![1]),
        ];

        for ((a, k), ans) in cases {
            assert_eq!(sliding_window_maximum(&a, k), ans);
        }
    }
}
