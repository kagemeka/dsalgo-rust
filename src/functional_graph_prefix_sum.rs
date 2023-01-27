use crate::functional_graph_basic_properties::*;

pub struct PrefixSum {
    s: Vec<usize>,
    start: usize,
}

impl PrefixSum {
    pub fn new(
        f: &[usize],
        src: usize,
        a: &[usize],
    ) -> Self {
        let (nodes, start) = functional_graph_prop(f, src);

        let mut s: Vec<_> = nodes.into_iter().map(|i| a[i].clone()).collect();

        for i in 1..s.len() {
            if i == start {
                continue;
            }

            s[i] += s[i - 1];
        }

        Self { s, start }
    }

    /// 0-indexed

    pub fn calc(
        &self,
        mut k: usize,
    ) -> usize {
        let j = self.start;

        let s = &self.s;

        if k < j {
            return s[k];
        }

        let mut res = 0;

        let l = s.len() - j;

        if j > 0 {
            res = s[j - 1];

            k -= j;
        }

        res += k / l * s.last().unwrap();

        k %= l;

        res += s[j + k];

        res
    }
}
