/// O(N)

pub fn table(size: usize) -> Vec<u8> {
    let mut l = vec![0; size];

    for i in 1..size {
        l[i] = l[i >> 1] + 1;
    }

    l
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
