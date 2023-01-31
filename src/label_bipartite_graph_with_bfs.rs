//! add a pattern of labels to nodes in the bipartite graph (true/false)
//! if the given graph is not bipartite, return None

/// given graph must be bidirected.

pub fn label_bipartite(g: &[Vec<usize>]) -> Option<Vec<bool>> {
    let n = g.len();

    let mut label = vec![false; n];

    let mut que = std::collections::VecDeque::new();

    let mut labeled = vec![false; n];

    for i in 0..n {
        if labeled[i] {
            continue;
        }

        labeled[i] = true;

        que.push_back(i);

        while let Some(u) = que.pop_front() {
            for &v in g[u].iter() {
                if !labeled[v] {
                    label[v] = label[u] ^ true;

                    labeled[v] = true;
                } else if label[v] == label[u] {
                    return None;
                }
            }
        }
    }

    debug_assert!(labeled.into_iter().fold(true, |x, y| x && y));

    Some(label)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // TODO:
    }
}
