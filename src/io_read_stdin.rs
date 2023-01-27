use crate::io_read_token::read_token;

pub fn read_stdin<T>() -> Result<T, <T as std::str::FromStr>::Err>
where
    T: std::str::FromStr,
{
    read_token(&mut std::io::stdin().lock())
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
