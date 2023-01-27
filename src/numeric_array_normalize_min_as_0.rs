use std::ops::*;

pub fn normalize<T>(mut a: Vec<T>) -> Vec<T>
where
    T: SubAssign + Ord + Clone,
{
    if a.len() == 0 {
        return a;
    }

    let mn = a.iter().min().unwrap().clone();

    for x in a.iter_mut() {
        *x -= mn.clone();
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![2, 4, 1];

        assert_eq!(normalize(a), [1, 3, 0]);
    }
}
