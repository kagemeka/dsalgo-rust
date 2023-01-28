/// unlike, u64, negative edge is possible.
/// inf + negative = inf.

pub fn floyd_warshall(
    inf: i64,
    mut g: Vec<Vec<i64>>,
) -> Vec<Vec<i64>> {
    let n = g.len();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if g[i][k] == inf || g[k][j] == inf {
                    continue;
                }

                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }

    g
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
