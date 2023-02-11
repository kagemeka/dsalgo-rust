pub fn connected_components(g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();

    let mut ids = vec![n; n];

    let mut id = 0;

    let mut st = vec![];

    for i in 0..n {
        if ids[i] != n {
            continue;
        }

        st.push(i);

        while let Some(u) = st.pop() {
            if ids[u] != n {
                continue;
            }

            ids[u] = id;

            for &v in g[u].iter() {
                if ids[v] == n {
                    st.push(v);
                }
            }
        }

        id += 1;
    }

    ids
}
