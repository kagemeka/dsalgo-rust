pub mod kp01 {

    //! 01 knapsack problems

    /// max sum of values such that their weights sum is `at most` capacity.

    pub fn knapsack_01(
        value_weight_pairs: &[(u64, u64)],
        capacity: u64,
    ) -> u64 {
        if value_weight_pairs.iter().map(|&(_, w)| w).sum::<u64>() <= capacity {
            value_weight_pairs.iter().map(|&(v, _)| v).sum()
        } else {
            let c = capacity as usize;

            // knapsack_01_table_just(value_weight_pairs, c + 1)
            //     .iter()
            //     .filter_map(|&x| x)
            //     .max()
            //     .unwrap()
            knapsack_01_table(value_weight_pairs, c + 1)[c]
        }
    }

    /// dp[i] := max sum of values such that their weights sum is `just` i.
    #[allow(dead_code)]

    pub(crate) fn knapsack_01_table_just(
        value_weight_pairs: &[(u64, u64)],
        size: usize,
    ) -> Vec<Option<u64>> {
        let mut max_value = vec![None; size];

        max_value[0] = Some(0);

        for &(v, w) in value_weight_pairs {
            let w = w as usize;

            for i in (w..size).rev() {
                if max_value[i - w].is_none() {
                    continue;
                }

                let nv = Some(max_value[i - w].unwrap() + v);

                if max_value[i].is_none() || nv > max_value[i] {
                    max_value[i] = nv;
                }
            }
        }

        max_value
    }

    /// dp[i] := max sum of values such that their weights sum is `at most` i.

    pub(crate) fn knapsack_01_table(
        value_weight_pairs: &[(u64, u64)],
        size: usize,
    ) -> Vec<u64> {
        let mut max_value = vec![0; size];

        for &(v, w) in value_weight_pairs {
            let w = w as usize;

            for i in (w..size).rev() {
                max_value[i] =
                    std::cmp::max(max_value[i], max_value[i - w] + v);
            }
        }

        max_value
    }

    /// max sum of values such that their weights sum is `at most` capacity.

    pub fn knapsack_01_small_weights_sum(
        value_weight_pairs: &[(u64, u64)],
        capacity: u64,
    ) -> u64 {
        knapsack_01(value_weight_pairs, capacity)
    }

    /// max sum of values such that their weights sum is `at most` capacity.

    pub fn knapsack_01_small_values_sum(
        value_weight_pairs: &[(u64, u64)],
        capacity: u64,
    ) -> u64 {
        dual::dual_knapsack_01_table_just(
            value_weight_pairs,
            value_weight_pairs.iter().map(|&(v, _)| v).sum::<u64>() as usize
                + 1,
        )
        .into_iter()
        .enumerate()
        .filter_map(|(v, min_w)| {
            if let Some(w) = min_w {
                if w <= capacity {
                    Some(v as u64)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .max()
        .unwrap()
    }

    use crate::upper_bound_on_slice::upper_bound;

    pub fn knapsack_01_meet_in_the_middle(
        value_weight_pairs: &[(u64, u64)],
        capacity: u64,
    ) -> u64 {
        fn enumerate_bits_brute_force(items: &[(u64, u64)]) -> Vec<(u64, u64)> {
            let n = items.len();

            let mut cand = vec![];

            for s in 0..1 << n {
                let mut value = 0;

                let mut weight = 0;

                for i in 0..n {
                    if s >> i & 1 == 0 {
                        continue;
                    }

                    let (v, w) = items[i];

                    value += v;

                    weight += w;
                }

                cand.push((value, weight));
            }

            cand.sort_by_key(|&(_, w)| w);

            for i in 0..(1 << n) - 1 {
                cand[i + 1].0 = std::cmp::max(cand[i].0, cand[i + 1].0);
            }

            cand
        }

        let n = value_weight_pairs.len();

        let a = enumerate_bits_brute_force(&value_weight_pairs[..n >> 1]);

        let b = enumerate_bits_brute_force(&value_weight_pairs[n >> 1..]);

        let b_weights = b.iter().map(|&(_, w)| w).collect::<Vec<_>>();

        let mut max_value = 0;

        for &(v, w) in a.iter() {
            if w > capacity {
                break;
            }

            let i = upper_bound(&b_weights, &(capacity - w));

            debug_assert!(i > 0);

            max_value = std::cmp::max(max_value, v + b[i - 1].0);
        }

        max_value
    }

    /// return max sum of values of at most k items
    /// whose sum of weights is at most w.

    pub fn knapsack_01_at_most_k(
        value_weight_pairs: &[(u64, u64)],
        k: u64,
        capacity: u64,
    ) -> u64 {
        let n = value_weight_pairs.len();

        let k = k as usize;

        let c = capacity as usize;

        assert!(k <= n);

        let mut max_value = vec![vec![0; c + 1]; k + 1];

        for &(v, w) in value_weight_pairs {
            let w = w as usize;

            for i in (0..k).rev() {
                for j in w..=c {
                    max_value[i + 1][j] = std::cmp::max(
                        max_value[i + 1][j],
                        max_value[i][j - w] + v,
                    );
                }
            }
        }

        max_value[k][c]
    }

    // TODO:
    pub fn few_kinds_of_weights() {}

    pub mod dual {

        //! duality of 01 knapsack.

        pub fn dual_knapsack_01(
            value_weight_pairs: &[(u64, u64)],
            target_value: u64,
        ) -> Result<u64, &'static str> {
            let s = value_weight_pairs.iter().map(|&(v, _)| v).sum::<u64>();

            let s = s as usize;

            let t = target_value as usize;

            if s < t {
                return Err("sum of values cannot achieve target value");
            }

            Ok(dual_knapsack_01_table(value_weight_pairs, t + 1)[t].unwrap())
        }

        /// dp[i] := min sum of weights such that their values sum is `just` i.

        pub(crate) fn dual_knapsack_01_table_just(
            value_weight_pairs: &[(u64, u64)],
            size: usize,
        ) -> Vec<Option<u64>> {
            let mut min_weight = vec![None; size];

            min_weight[0] = Some(0);

            for &(v, w) in value_weight_pairs {
                let v = v as usize;

                for i in (v..size).rev() {
                    if min_weight[i - v].is_none() {
                        continue;
                    }

                    let nw = Some(min_weight[i - v].unwrap() + w);

                    if min_weight[i].is_none() || nw < min_weight[i] {
                        min_weight[i] = nw;
                    }
                }
            }

            min_weight
        }

        /// dp[i] := min sum of weights such that their values sum is `at least`
        /// i.

        pub(crate) fn dual_knapsack_01_table(
            value_weight_pairs: &[(u64, u64)],
            size: usize,
        ) -> Vec<Option<u64>> {
            let mut min_weight = vec![None; size];

            min_weight[0] = Some(0);

            for &(v, w) in value_weight_pairs {
                let v = v as usize;

                for i in (v..size).rev() {
                    if min_weight[i - v].is_none() {
                        continue;
                    }

                    let nw = Some(min_weight[i - v].unwrap() + w);

                    if min_weight[i].is_none() || nw < min_weight[i] {
                        min_weight[i] = nw;
                    }
                }

                for i in (1..size).rev() {
                    if min_weight[i].is_none() {
                        continue;
                    }

                    if min_weight[i - 1].is_none()
                        || min_weight[i] < min_weight[i - 1]
                    {
                        min_weight[i - 1] = min_weight[i];
                    }
                }
            }

            min_weight
        }

        /// dp[i] := min sum of weights such that their values sum is `at least`
        /// i.
        #[allow(dead_code)]

        pub(crate) fn dual_knapsack_01_table_fast(
            value_weight_pairs: &[(u64, u64)],
            size: usize,
        ) -> Vec<Option<u64>> {
            let mut min_weight = vec![None; size];

            min_weight[0] = Some(0);

            for &(v, w) in value_weight_pairs {
                let v = v as usize;

                for i in (0..size).rev() {
                    if min_weight[i].is_none() {
                        continue;
                    }

                    let nw = Some(min_weight[i].unwrap() + w);

                    let j = std::cmp::min(i + v, size - 1);

                    if min_weight[j].is_none() || nw < min_weight[j] {
                        min_weight[j] = nw;
                    }
                }
            }

            for i in (1..size).rev() {
                if min_weight[i].is_none() {
                    continue;
                }

                if min_weight[i - 1].is_none()
                    || min_weight[i] < min_weight[i - 1]
                {
                    min_weight[i - 1] = min_weight[i];
                }
            }

            min_weight
        }
    }
}

/// accept negative weights.

pub mod neg_weight {}

pub mod unbouneded {

    //! unbounded knapsack problems

    /// max sum of values such that their weights sum is `at most` capacity.

    pub fn unbounded_knapsack(
        value_weight_pairs: &[(u64, u64)],
        capacity: u64,
    ) -> u64 {
        let c = capacity as usize;

        unbounded_knapsack_table(value_weight_pairs, c + 1)[c]
    }

    /// dp[i] := max sum of values such that their weights sum is `at most` i.

    pub(crate) fn unbounded_knapsack_table(
        value_weight_pairs: &[(u64, u64)],
        size: usize,
    ) -> Vec<u64> {
        let mut max_value = vec![0; size];

        for &(v, w) in value_weight_pairs {
            let w = w as usize;

            for i in w..size {
                max_value[i] =
                    std::cmp::max(max_value[i], max_value[i - w] + v);
            }
        }

        max_value
    }

    pub mod dual {

        /// dp[i] := min sum of weights such that their values sum is `just` i.

        pub(crate) fn dual_unbounded_knapsack_table_just(
            value_weight_pairs: &[(u64, u64)],
            size: usize,
        ) -> Vec<Option<u64>> {
            let mut min_weight = vec![None; size];

            min_weight[0] = Some(0);

            for &(v, w) in value_weight_pairs {
                let v = v as usize;

                for i in v..size {
                    if min_weight[i - v].is_none() {
                        continue;
                    }

                    let nw = Some(min_weight[i - v].unwrap() + w);

                    if min_weight[i].is_none() || nw < min_weight[i] {
                        min_weight[i] = nw;
                    }
                }
            }

            min_weight
        }
    }
}

pub mod kp3d {}

pub mod kp_ndim {

    //! n-dimensional knapsack problems
    //! general linear programming
}

// TODO:
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
