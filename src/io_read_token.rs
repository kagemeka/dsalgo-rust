pub fn read_token<R, T>(
    reader: &mut R
) -> Result<T, <T as std::str::FromStr>::Err>
where
    R: std::io::Read,
    T: std::str::FromStr,
{
    use std::io::Read;

    reader
        .by_ref()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<T>()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
