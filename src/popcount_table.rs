pub fn popcount(size: usize) -> Vec<u8> {
    let mut count = vec![0; size];

    for i in 1..size {
        count[i] = count[i >> 1] + (i & 1) as u8;
    }

    count
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(popcount(8), [0, 1, 1, 2, 1, 2, 2, 3]);
        // dbg!(popcount(1 << 8));
    }
}
