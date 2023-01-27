pub fn a_star(
    is_path: &[Vec<bool>],
    sy: usize,
    sx: usize,
    gy: usize,
    gx: usize,
) -> Option<u32> {
    use std::cmp::Reverse;

    let hf = |y, x| {
        ((y as isize - gy as isize).abs() + (x as isize - gx as isize).abs())
            as u32
    };

    const INF: u32 = std::u32::MAX;

    let h = is_path.len();

    let w = is_path[0].len();

    let mut cost = vec![vec![INF; w]; h];

    cost[sy][sx] = 0;

    let mut que = std::collections::BinaryHeap::new();

    que.push((Reverse(hf(sy, sx)), 0, sy, sx));

    let on_path = |y, x| {
        if !(0 <= y && y < h as isize && 0 <= x && x < w as isize) {
            return false;
        }

        is_path[y as usize][x as usize]
    };

    let dyx = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some((Reverse(_), c, y, x)) = que.pop() {
        if y == gy && x == gx {
            return Some(c);
        }

        if c > cost[y][x] {
            continue;
        }

        for &(dy, dx) in dyx.iter() {
            let ny = y as isize + dy;

            let nx = x as isize + dx;

            if !on_path(ny, nx) {
                continue;
            }

            let ny = ny as usize;

            let nx = nx as usize;

            let cv = c + 1;

            if cv >= cost[ny][nx] {
                continue;
            }

            cost[ny][nx] = cv;

            let s = hf(ny, nx) + cv;

            que.push((Reverse(s), cv, ny, nx));
        }
    }

    None
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
