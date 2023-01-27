//! inspired by numpy.flatnonzero

use crate::convert_to_bool::ToBool;

pub fn flat_nonzero<T: ToBool>(a: &[T]) -> Vec<usize> {
    a.iter()
        .enumerate()
        .filter_map(|(i, x)| if x.to_bool() { Some(i) } else { None })
        .collect()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
