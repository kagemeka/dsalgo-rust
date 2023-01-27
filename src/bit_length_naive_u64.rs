/// O(\log{N})

pub fn naive(mut n: u64) -> u8 {
    let mut l = 0;

    while n > 0 {
        n >>= 1;

        l += 1;
    }

    l
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
