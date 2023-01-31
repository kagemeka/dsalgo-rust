use crate::functional_graph_basic_properties::functional_graph_prop;

pub struct PrefixAggregation<T, F> {
    cum: Vec<T>,
    start: usize,
    op: F,
}

impl<T, F> PrefixAggregation<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    pub fn new(
        f: &[usize],
        src: usize,
        a: &[T],
        op: F,
    ) -> Self {
        let (nodes, start) = functional_graph_prop(f, src);

        let mut b: Vec<_> = nodes.into_iter().map(|i| a[i].clone()).collect();

        for i in 1..b.len() {
            if i == start {
                continue;
            }

            b[i] = op(&b[i - 1], &b[i]);
        }

        Self { cum: b, start, op }
    }

    // /// 0-indexed
    // pub fn calc(&self, mut k: usize) -> T {
    //     if k < self.start {
    //         return self.cum[k];
    //     }
    //     let l = self.cum.len() - self.start;
    //     let mut res = (n - k) % l
    //     // if self.start > 0 {
    //     // }
    // }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
