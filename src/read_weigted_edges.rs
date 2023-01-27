use crate::io_read_stdin_direct::*;

pub fn read_weighted_edges<T: std::str::FromStr>(
    m: usize,
    offset: usize,
) -> Vec<(usize, usize, T)> {
    (0..m)
        .map(|_| {
            (read::<usize>() - offset, read::<usize>() - offset, read::<T>())
        })
        .collect()
}
