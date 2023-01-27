use crate::next_combination_bits::next_combination;

pub fn combinations(
    n: usize,
    k: usize,
) -> Vec<usize> {
    assert!(k <= n);

    if k == 0 {
        return vec![0];
    }

    let mut res = vec![];

    let mut s: usize = (1 << k) - 1;

    let lim = 1 << n;

    while s < lim {
        res.push(s);

        s = next_combination(s);
    }

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        dbg!(std::mem::size_of::<usize>());

        for s in combinations(8, 4) {
            println!("{:08b} {}", s, s);
        }

        let cases = vec![(
            5,
            3,
            vec![
                0b00111, 0b01011, 0b01101, 0b01110, 0b10011, 0b10101, 0b10110,
                0b11001, 0b11010, 0b11100,
            ],
        )];

        for (n, k, ans) in cases {
            assert_eq!(combinations(n, k), ans);
        }
    }
}
