pub fn stepping_numbers(n: usize) -> Vec<usize> {
    assert!(n > 0);

    let mut res: Vec<_> = (0..10).collect();

    let mut i = 1;

    while i < res.len() {
        let x = res[i];

        if x >= n {
            break;
        }

        let r = x % 10;

        if r > 0 {
            res.push(x * 10 + i - 1);
        }

        if r < 9 {
            res.push(x * 10 + i + 1);
        }

        i += 1;
    }

    res[..i].to_vec()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        dbg!(stepping_numbers(100));
    }
}
