use crate::io_read_stdin_direct::*;

pub fn read_matrix<T: std::str::FromStr>(
    h: usize,
    w: usize,
) -> Vec<Vec<T>> {
    let mut g = Vec::with_capacity(h);

    for _ in 0..h {
        g.push((0..w).map(|_| read::<T>()).collect());
    }

    g
}
