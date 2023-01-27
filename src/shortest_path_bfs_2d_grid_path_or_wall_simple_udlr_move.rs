pub fn bfs_grid(
    inf: u32,
    is_path: &[Vec<bool>],
    sy: usize,
    sx: usize,
) -> Vec<Vec<u32>> {
    let h = is_path.len();

    let w = is_path[0].len();

    let mut dist = vec![vec![inf; w]; h];

    dist[sy][sx] = 0;

    let mut que = std::collections::VecDeque::new();

    que.push_back((sy, sx));

    let on_path = |y, x| {
        if !(0 <= y && y < h as isize && 0 <= x && x < w as isize) {
            return false;
        }

        is_path[y as usize][x as usize]
    };

    let dyx = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some((y, x)) = que.pop_front() {
        let d = dist[y][x] + 1;

        for &(dy, dx) in dyx.iter() {
            let ny = y as isize + dy;

            let nx = x as isize + dx;

            if !on_path(ny, nx) {
                continue;
            }

            let ny = ny as usize;

            let nx = nx as usize;

            if dist[ny][nx] != inf {
                continue;
            }

            dist[ny][nx] = d;

            que.push_back((ny, nx));
        }
    }

    dist
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
