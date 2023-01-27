pub fn floyd_warshall(mut g: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let n = g.len();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
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
