use std::ops::*;

pub fn tsp<T>(
    inf: T,
    g: &[Vec<T>],
    src: usize,
) -> T
where
    T: Clone + Add<Output = T> + Ord,
{
    let n = g.len();

    let mut dist = vec![vec![inf.clone(); n]; 1 << n];

    for i in 0..n {
        if i == src {
            continue;
        }

        dist[1 << i][i] = g[src][i].clone();
    }

    for s in 0..1 << n {
        for i in 0..n {
            if s >> i & 1 == 1 {
                continue;
            }

            let t = s | 1 << i;

            for j in 0..n {
                if !s >> j & 1 == 1 {
                    continue;
                }

                if dist[s][j] == inf || g[j][i] == inf {
                    continue;
                }

                dist[t][i] = dist[t][i]
                    .clone()
                    .min(dist[s][j].clone() + g[j][i].clone());
            }
        }
    }

    dist.last().unwrap()[src].clone()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
