use std::ops::*;

pub fn normalize<T>(
    mut a: Vec<T>,
    offset: T,
) -> Vec<T>
where
    T: Add<Output = T> + Sub<Output = T> + Ord + From<i32> + Clone,
{
    let mn = a.iter().min().unwrap_or(&0.into()).clone();

    for x in a.iter_mut() {
        *x = x.clone() - mn.clone() + offset.clone();
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![2, 4, 1];

        assert_eq!(normalize(a, 3), [4, 6, 3]);
    }
}
