use crate::strongly_connected_components_transpose::transpose;

pub fn scc(adjacency_list: &[Vec<usize>]) -> Vec<usize> {
    let g = adjacency_list;

    let n = g.len();

    let mut state = vec![0; n];

    let mut post_order = vec![];

    let mut st = vec![];

    for i in 0..n {
        if state[i] == n {
            continue;
        }

        st.push(i as isize);

        while let Some(u) = st.pop() {
            if u < 0 {
                post_order.push(!u as usize);

                continue;
            }

            if state[u as usize] == n {
                continue;
            }

            st.push(!u);

            state[u as usize] = n;

            for &v in g[u as usize].iter() {
                if state[v] == 0 {
                    st.push(v as isize);
                }
            }
        }
    }

    let g = transpose(&g);

    let mut label = 0;

    let mut st = vec![];

    for i in post_order.into_iter().rev() {
        if state[i] != n {
            continue;
        }

        state[i] = label;

        st.push(i);

        while let Some(u) = st.pop() {
            for &v in g[u].iter() {
                if state[v] != n {
                    continue;
                }

                state[v] = label;

                st.push(v);
            }
        }

        label += 1;
    }

    state
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (6, vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)]),
            vec![3, 1, 2, 3, 1, 0],
        )];

        for ((n, edges), ans) in cases {
            let mut g = vec![vec![]; n];

            for (u, v) in edges {
                g[u].push(v);
            }

            assert_eq!(scc(&g), ans);
        }
    }
}
