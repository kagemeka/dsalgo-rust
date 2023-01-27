pub fn argsort(mut a: Vec<usize>) -> Vec<usize> {
    let n = a.len();

    let mut m = *a.iter().min().unwrap();

    for x in a.iter_mut() {
        *x -= m;
    }

    m = *a.iter().max().unwrap();

    let mut arg = vec![0; m + 1];

    for &x in a.iter() {
        arg[x] += 1;
    }

    for i in 0..m {
        arg[i + 1] += arg[i];
    }

    let mut res = vec![0; n];

    for (i, &x) in a.iter().enumerate().rev() {
        arg[x] -= 1;

        res[arg[x]] = i;
    }

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![7, 2, 1, 3, 2, 1];

        assert_eq!(argsort(a), [2, 5, 1, 4, 3, 0]);

        let a = vec![4, 3, 1, 6, 0, 6, 3];

        assert_eq!(argsort(a), [4, 2, 1, 6, 0, 3, 5]);
    }
}
