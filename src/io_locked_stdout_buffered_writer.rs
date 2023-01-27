pub fn locked_stdout_buf_writer(
) -> std::io::BufWriter<std::io::StdoutLock<'static>> {
    let stdout = Box::leak(Box::new(std::io::stdout()));

    std::io::BufWriter::new(stdout.lock())
}

#[cfg(test)]

mod tests {

    #[test]

    fn test_locked_stdin_buf_writer() {
        use std::io::Write;

        use super::locked_stdout_buf_writer;

        let mut writer = locked_stdout_buf_writer();

        writeln!(writer, "Hello, world!").unwrap();

        writer.flush().unwrap();
    }
}
