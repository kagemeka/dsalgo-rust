use crate::group_theory;

pub trait PascalTriangle<T>: group_theory::Monoid<T> + group_theory::Default<T>
where
    Self: Sized,
    T: crate::group_theory::BinaryOperationIdentifier,
{
    fn pascal_triangle(n: usize) -> Vec<Vec<Self>>;
}

impl<S, T> PascalTriangle<T> for S
where
    S: group_theory::Monoid<T> + group_theory::Default<T> + Clone,
    T: crate::group_theory::BinaryOperationIdentifier,
{
    fn pascal_triangle(n: usize) -> Vec<Vec<S>> {
        let mut p: Vec<Vec<S>> = vec![vec![S::identity(); n]; n];
        for i in 0..n {
            p[i][0] = S::default();
        }
        for i in 1..n {
            for j in 1..i + 1 {
                p[i][j] = S::operate(&p[i - 1][j], &p[i - 1][j - 1]);
            }
        }
        p
    }
}
