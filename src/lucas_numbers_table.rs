pub fn lucas_numbers(size: usize) -> Vec<usize> {
    let mut l = vec![0; size];

    l[0] = 2;

    l[1] = 1;

    for i in 2..size {
        l[i] = l[i - 1] + l[i - 2];
    }

    l
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        const ANS: &[usize] = &[
            2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521, 843, 1364,
            2207, 3571, 5778, 9349, 15127, 24476, 39603, 64079, 103682, 167761,
            271443, 439204, 710647, 1149851, 1860498, 3010349, 4870847,
            7881196, 12752043, 20633239, 33385282, 54018521, 87403803,
        ];

        let n = ANS.len();

        assert_eq!(lucas_numbers(n), ANS);
    }
}
