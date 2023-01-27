use std::ops::*;

pub fn cumprod<T>(mut a: Vec<T>) -> Vec<T>
where
    T: Mul<Output = T> + Clone,
{
    for i in 0..a.len() - 1 {
        a[i + 1] = a[i + 1].clone() * a[i].clone()
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut a: Vec<_> = (1..10).collect();

        a = cumprod(a);

        assert_eq!(a, [1, 2, 6, 24, 120, 720, 5040, 40320, 362880]);
    }
}
