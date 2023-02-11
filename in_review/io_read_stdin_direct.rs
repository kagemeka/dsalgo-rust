pub fn read<T: std::str::FromStr>() -> T {
    use std::io::Read;

    std::io::stdin()
        .lock()
        .by_ref()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<T>()
        .ok()
        .unwrap()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
