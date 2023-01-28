pub struct KthFrom {
    cycles: Vec<Vec<usize>>,
    cycle_id: Vec<usize>,
    idx_in_cycle: Vec<usize>,
}

impl KthFrom {
    pub fn new(f: &[usize]) -> Self {
        let mut cycles = vec![];

        let n = f.len();

        let mut cycle_id = vec![n; n];

        let mut idx_in_cycle = vec![n; n];

        for mut i in 0..n {
            if cycle_id[i] != n {
                continue;
            }

            let id = cycles.len();

            let mut cycle = vec![];

            for j in 0..n {
                if cycle_id[i] != n {
                    break;
                }

                cycle_id[i] = id;

                idx_in_cycle[i] = j;

                cycle.push(i);

                i = f[i];
            }

            cycles.push(cycle);
        }

        Self { cycles, cycle_id, idx_in_cycle }
    }

    pub fn get(
        &self,
        i: usize,
        k: usize,
    ) -> usize {
        let cycle = &self.cycles[self.cycle_id[i]];

        cycle[(k + self.idx_in_cycle[i]) % cycle.len()]
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
