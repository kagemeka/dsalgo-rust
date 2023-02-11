pub fn eliminate(mut a: Vec<u64>) -> Vec<u64> {
    let n = a.len();

    let mut r = 0;

    for k in (0..64).rev() {
        let mut i = r;

        while i < n {
            if a[i] >> k & 1 == 1 {
                break;
            }

            i += 1;
        }

        if i == n {
            continue;
        }

        a.swap(r, i);

        for i in 0..n {
            if i != r && a[i] >> k & 1 == 1 {
                a[i] ^= a[r];
            }
        }

        r += 1;
    }

    a
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![0b11001, 0b10101, 0b00010];

        assert_eq!(eliminate(a), [0b10101, 0b01100, 0b00010]);
    }
}
