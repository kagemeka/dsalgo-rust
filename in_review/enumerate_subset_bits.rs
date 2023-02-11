pub fn enumerate_subsets(s: usize) -> Vec<usize> {
    let mut subsets = vec![];

    let mut t = s;

    loop {
        subsets.push(t);

        if t == 0 {
            subsets.reverse();

            return subsets;
        }

        t = (t - 1) & s;
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(0b101, vec![0b000, 0b001, 0b100, 0b101])];

        for (s, ans) in cases {
            assert_eq!(enumerate_subsets(s), ans);
        }
    }
}
