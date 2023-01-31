/// (preorder, cycle_start_index)

pub fn functional_graph_prop(
    f: &[usize],
    src: usize,
) -> (Vec<usize>, usize) {
    let n = f.len();

    let mut idx = vec![n; n];

    let mut preorder = Vec::with_capacity(n);

    let mut x = src;

    for i in 0..n + 1 {
        if idx[x] == n {
            idx[x] = i;

            preorder.push(x);

            x = f[x];

            continue;
        }

        return (preorder, idx[x]);
    }

    panic!();
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
