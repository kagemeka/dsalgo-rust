use std::ops::*;

pub fn pascal_triangle<F, T>(
    f: F,
    zero: T,
    one: T,
    size: usize,
) -> Vec<Vec<T>>
where
    T: Add<Output = T> + Clone,
    F: Fn(T, T) -> T,
{
    let mut p: Vec<Vec<T>> = vec![vec![zero.clone(); size]; size];

    for i in 0..size {
        p[i][0] = one.clone();
    }

    for i in 1..size {
        for j in 1..=i {
            p[i][j] = f(p[i - 1][j - 1].clone(), p[i - 1][j].clone());
        }
    }

    p
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let p = pascal_triangle(|a, b| a / 4.0 + b, 0.0, 1.0, 6);

        println!("{:#?}", p);
        // assert_eq!(p, vec![
        //     vec![1, 0, 0, 0, 0, 0],
        //     vec![1, 1, 0, 0, 0, 0],
        //     vec![1, 2, 1, 0, 0, 0],
        //     vec![1, 3, 3, 1, 0, 0],
        //     vec![1, 4, 6, 4, 1, 0],
        //     vec![1, 5, 10, 10, 5, 1],
        // ]);
    }
}
