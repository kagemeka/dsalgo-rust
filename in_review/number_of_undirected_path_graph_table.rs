use std::ops::*;

pub fn number_of_path_graph<T>(size: usize) -> Vec<T>
where
    T: Clone + Mul<Output = T> + From<usize>,
{
    let mut f: Vec<T> = vec![1.into(); size];

    f[0] = 0.into();

    for i in 3..size {
        f[i] = f[i - 1].clone() * i.into();
    }

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let f = number_of_path_graph::<usize>(7);

        assert_eq!(f, [0, 1, 1, 3, 12, 60, 360]);
    }
}
