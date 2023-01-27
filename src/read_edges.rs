use crate::io_read_stdin_direct::read;

pub fn read_edges(
    m: usize,
    offset: usize,
) -> Vec<(usize, usize)> {
    (0..m)
        .map(|_| (read::<usize>() - offset, read::<usize>() - offset))
        .collect()
}
