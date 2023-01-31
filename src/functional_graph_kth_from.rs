// from a initialized node
pub struct FunctionalGraphKthFrom {
    pub path: Vec<usize>,
    pub cycle_size: usize,
    pub cycle_start_idx: usize,
}

impl FunctionalGraphKthFrom {
    pub fn new(
        f: &[usize],
        src: usize,
    ) -> Self {
        let n = f.len();

        let mut path = vec![];

        let mut order = vec![n; n];

        let mut i = src;

        let mut j = 0;

        loop {
            if order[i] != n {
                return Self {
                    path,
                    cycle_size: j - order[i],
                    cycle_start_idx: order[i],
                };
            }

            order[i] = j;

            path.push(i);

            i = f[i];

            j += 1;
        }
    }

    pub fn get(
        &self,
        mut k: usize,
    ) -> usize {
        let s = self.cycle_start_idx;

        if k >= s {
            k = (k - s) % self.cycle_size + s;
        }

        self.path[k]
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((vec![1, 2, 0, 1, 5, 4], 3, 5), 2),
            ((vec![1, 2, 3, 4, 2, 1, 3, 4], 0, 1), 1),
        ];

        for ((f, src, k), ans) in cases {
            let f = FunctionalGraphKthFrom::new(&f, src);

            assert_eq!(f.get(k), ans);
        }
    }
}
