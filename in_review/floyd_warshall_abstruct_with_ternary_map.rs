/// adjacency matrix

type G<T> = Vec<Vec<T>>;

pub fn floyd_warshall<T: Clone, F: FnMut(T, T, T) -> T>(
    mut g: G<T>,
    mut f: F,
) -> G<T> {
    let n = g[0].len();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                g[i][j] = f(g[i][j].clone(), g[i][k].clone(), g[k][j].clone());
            }
        }
    }

    g
}
