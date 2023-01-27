use crate::io_buffered_read_wrapper::*;

pub fn locked_stdin_reader() -> ReadWrapper<std::io::StdinLock<'static>> {
    let stdin = Box::leak(Box::new(std::io::stdin()));

    ReadWrapper::new(stdin.lock())
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
