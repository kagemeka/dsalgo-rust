pub fn csgraph_from_dense<T: Eq>(
    inf: &T,
    g: Vec<Vec<T>>,
) -> Vec<(usize, usize, T)> {
    let mut cs = Vec::new();

    for (u, row) in g.into_iter().enumerate() {
        for (v, w) in row.into_iter().enumerate() {
            if &w == inf {
                continue;
            }

            cs.push((u, v, w));
        }
    }

    cs
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let inf = 1 << 30;

        let cases = vec![(
            vec![vec![0, 1, inf], vec![inf, 0, 2], vec![inf, 3, 0]],
            vec![
                (0, 0, 0),
                (0, 1, 1),
                (1, 1, 0),
                (1, 2, 2),
                (2, 1, 3),
                (2, 2, 0),
            ],
        )];

        for (g, ans) in cases {
            assert_eq!(csgraph_from_dense(&inf, g), ans);
        }
    }
}
