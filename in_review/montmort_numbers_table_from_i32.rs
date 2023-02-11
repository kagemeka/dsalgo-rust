use std::ops::*;

pub fn montmort_numbers<T>(size: usize) -> Vec<T>
where
    T: From<i32> + Clone + Mul<Output = T> + Add<Output = T>,
{
    let mut a: Vec<T> = vec![0.into(); size];

    a[0] = 1.into();

    for i in 2..size {
        a[i] = (a[i - 2].clone() + a[i - 1].clone()) * (i as i32 - 1).into();
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::const_montmort_numbers_usize::MONTMORT_NUMBERS;

        let n = MONTMORT_NUMBERS.len();

        let res: Vec<_> = montmort_numbers::<i64>(n)
            .into_iter()
            .map(|x| x as usize)
            .collect();

        assert_eq!(res, MONTMORT_NUMBERS);
    }
}
