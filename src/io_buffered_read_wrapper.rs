pub struct ReadWrapper<R> {
    reader: R,
    tokens: Vec<String>,
}

impl<R> ReadWrapper<R> {
    pub fn new(reader: R) -> Self { Self { reader, tokens: vec![] } }
}

impl<R: std::io::BufRead> ReadWrapper<R> {
    pub fn read<T: std::str::FromStr>(
        &mut self
    ) -> Result<T, <T as std::str::FromStr>::Err> {
        while self.tokens.is_empty() {
            let mut buf = String::new();

            self.reader.read_line(&mut buf).unwrap();

            self.tokens =
                buf.split_whitespace().map(str::to_string).rev().collect();
        }

        self.tokens.pop().unwrap().parse::<T>()
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test_read_wrapper() {
        use super::ReadWrapper;

        let stdin = std::io::stdin();

        let _reader = ReadWrapper::new(stdin.lock());
    }
}
