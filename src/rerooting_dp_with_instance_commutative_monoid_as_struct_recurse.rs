pub trait Monoid {
    type T;

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T;

    fn e(&self) -> Self::T;
}

pub trait Edge {
    fn to(&self) -> usize;
}

pub struct ReRootingDP<'a, M: Monoid, E, F> {
    g: &'a [Vec<E>],
    m: M,
    f: F,
    rev_edge: Vec<Option<&'a E>>,
    dp_from_childs: Vec<M::T>,
    dp: Vec<M::T>,
}

impl<'a, M: Monoid, E, F: Fn(&E, M::T) -> M::T> ReRootingDP<'a, M, E, F>
where
    M::T: Clone + std::fmt::Debug,
    E: Clone + Edge,
{
    fn new(
        g: &'a [Vec<E>],
        m: M,
        f: F,
    ) -> Self {
        let n = g.len();

        let dp_from_childs = vec![m.e(); n];

        let dp = vec![m.e(); n];

        let rev_edge = vec![None; n];

        Self { g, m, f, rev_edge, dp_from_childs, dp }
    }

    pub fn calc(
        g: &'a [Vec<E>],
        m: M,
        f: F,
    ) -> Vec<M::T> {
        let mut wrapper = Self::new(g, m, f);

        wrapper.tree_dp(0, 0);

        wrapper.reroot(0, 0, wrapper.m.e());

        wrapper.dp
    }

    fn tree_dp(
        &mut self,
        u: usize,
        parent: usize,
    ) {
        for e in self.g[u].iter() {
            let v = e.to();

            if v == parent {
                self.rev_edge[u] = Some(e);

                continue;
            }

            self.tree_dp(v, u);

            self.dp_from_childs[u] = self.m.op(
                self.dp_from_childs[u].clone(),
                (self.f)(&e, self.dp_from_childs[v].clone()),
            );
        }
    }

    fn reroot(
        &mut self,
        u: usize,
        parent: usize,
        x: M::T,
    ) {
        self.dp[u] = self.m.op(x.clone(), self.dp_from_childs[u].clone());

        let n = self.g[u].len();

        let mut dp_r = vec![self.m.e(); n + 1];

        for (i, e) in self.g[u].iter().enumerate().rev() {
            let v = e.to();

            dp_r[i] = if v == parent {
                dp_r[i + 1].clone()
            } else {
                self.m.op(
                    (self.f)(e, self.dp_from_childs[v].clone()),
                    dp_r[i + 1].clone(),
                )
            };
        }

        let mut dp_l = self.m.e();

        for (i, e) in self.g[u].iter().enumerate() {
            let v = e.to();

            if v == parent {
                continue;
            }

            let y = (self.f)(
                self.rev_edge[v].unwrap(),
                self.m.op(
                    x.clone(),
                    self.m.op(dp_l.clone(), dp_r[i + 1].clone()),
                ),
            );

            dp_l = self.m.op(dp_l, (self.f)(e, self.dp_from_childs[v].clone()));

            self.reroot(v, u, y);
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // https://atcoder.jp/contests/abc222/tasks/abc222_f
        struct M;

        impl Monoid for M {
            type T = u64;

            fn op(
                &self,
                lhs: Self::T,
                rhs: Self::T,
            ) -> Self::T {
                lhs.max(rhs)
            }

            fn e(&self) -> Self::T { 0 }
        }

        #[derive(Clone)]

        struct E {
            to: usize,
            weight: u64,
        }

        impl E {
            pub fn new(
                to: usize,
                weight: u64,
            ) -> Self {
                Self { to, weight }
            }
        }

        impl Edge for E {
            fn to(&self) -> usize { self.to }
        }

        let g = vec![
            vec![E::new(1, 2)],
            vec![E::new(0, 2), E::new(2, 3)],
            vec![E::new(1, 3)],
        ];

        let d = vec![1, 2, 3];

        let map = |e: &E, x: u64| -> u64 { e.weight + x.max(d[e.to()]) };

        let res = ReRootingDP::calc(&g, M {}, &map);

        assert_eq!(res, [8, 6, 6]);
    }

    #[test]

    fn test_abc220_f() {
        // ref: https://atcoder.jp/contests/abc220/tasks/abc220_f
        struct M;

        impl Monoid for M {
            type T = (usize, usize);

            fn e(&self) -> Self::T { (0, 0) }

            fn op(
                &self,
                l: Self::T,
                r: Self::T,
            ) -> Self::T {
                (l.0 + r.0, l.1 + r.1)
            }
        }

        #[derive(Clone)]

        struct E(usize);

        impl Edge for E {
            fn to(&self) -> usize { self.0 }
        }

        let cases = vec![
            ((3, vec![(1, 2), (2, 3)]), vec![3, 2, 3]),
            ((2, vec![(1, 2)]), vec![1, 1]),
            (
                (6, vec![(1, 6), (1, 5), (1, 3), (1, 4), (1, 2)]),
                vec![5, 9, 9, 9, 9, 9, 9],
            ),
        ];

        for ((n, edges), ans) in cases {
            let mut g = vec![vec![]; n];

            for (u, v) in edges {
                let u = u - 1;

                let v = v - 1;

                g[u].push(E(v));

                g[v].push(E(u));
            }

            let res = ReRootingDP::calc(&g, M {}, |_, x: (usize, usize)| {
                (x.0 + x.1 + 1, x.1 + 1)
            });

            for i in 0..n {
                assert_eq!(res[i].0, ans[i]);
            }
        }
    }
}
