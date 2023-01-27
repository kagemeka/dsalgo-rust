pub fn transpose(g: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = g.len();

    let mut t = vec![vec![]; n];

    for (i, edges) in g.iter().enumerate() {
        for &j in edges.iter() {
            t[j].push(i);
        }
    }

    t
}
