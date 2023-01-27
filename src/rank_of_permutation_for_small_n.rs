/// O(N)

pub fn permutation_rank(p: &[usize]) -> usize {
    let n = p.len();

    let mut s = (1usize << n) - 1;

    let mut fact = 1;

    let mut rank = 0;

    for (i, &p) in p.iter().rev().enumerate() {
        rank += (p - (s & ((1 << p) - 1)).count_ones() as usize) * fact;

        s &= !(1 << p);

        fact *= i + 1;
    }

    rank
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(vec![0, 2, 1], 1), (vec![2, 0, 1], 4)];

        for (p, r) in cases {
            assert_eq!(permutation_rank(&p), r);
        }
    }
}
