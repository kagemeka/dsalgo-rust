use std::ops::*;

pub fn tsp<T>(
    inf: T,
    g: &[Vec<T>],
    src: usize,
) -> T
where
    T: Clone + Add<Output = T> + Ord + From<i32>,
{
    let n = g.len();

    let mut dist = vec![vec![inf.clone(); n]; 1 << n];

    dist[1 << src][src] = 0.into();

    for s in 0..1 << n {
        for i in 0..n {
            if !s >> i & 1 == 1 {
                continue;
            }

            let t = s & !(1 << i);

            for j in 0..n {
                if !t >> j & 1 == 1 {
                    continue;
                }

                if dist[t][j] == inf || g[j][i] == inf {
                    continue;
                }

                dist[s][i] = dist[s][i]
                    .clone()
                    .min(dist[t][j].clone() + g[j][i].clone());
            }
        }
    }

    let mut mn = inf.clone();

    let s = (1 << n) - 1;

    for i in 0..n {
        if i == src || dist[s][i] == inf || g[i][src] == inf {
            continue;
        }

        mn = mn.min(dist[s][i].clone() + g[i][src].clone());
    }

    mn
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
