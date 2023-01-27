use crate::factorial_tables_frequent_ops_modular_usize::*;

pub struct CatalanNumber(FactorialTablesFrequentOps);

impl CatalanNumber {
    pub fn new(
        p: usize,
        less_than: usize,
    ) -> Self {
        Self(FactorialTablesFrequentOps::new(p, less_than << 1))
    }

    pub fn calc(
        &self,
        n: usize,
    ) -> usize {
        self.0.fact[n << 1] * self.0.inv_fact[n] % self.0.p
            * self.0.inv_fact[n + 1]
            % self.0.p
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::catalan_numbers_constant::*;

        let p = 1_000_000_007;

        let f = CatalanNumber::new(p, 100);

        for i in 0..CATALAN_NUMBERS.len() {
            assert_eq!(f.calc(i), CATALAN_NUMBERS[i] % p);
        }
    }
}
