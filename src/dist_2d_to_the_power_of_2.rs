use std::ops::*;

pub fn dist2<T>(
    x0: T,
    y0: T,
    x1: T,
    y1: T,
) -> T
where
    T: Sub<Output = T> + Clone + Mul<Output = T> + Add<Output = T>,
{
    let dx = x1 - x0;

    let dy = y1 - y0;

    dx.clone() * dx + dy.clone() * dy
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(dist2(-1, 1, 2, 3), 13);
    }
}
