#[macro_export]
#[allow(unused_macros)]

macro_rules! read_vec {
    ($reader:ident, $type:ty, $n:expr) => {
        (0..$n)
            .map(|_| $reader.read::<$type>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    };
}

#[cfg(test)]

mod tests {

    #[test]

    fn write_macro() {
        use std::io::Write;

        use crate::io_locked_stdout_buffered_writer::locked_stdout_buf_writer;

        let mut writer = locked_stdout_buf_writer();

        let mut v = vec![];

        v.push(1);

        v.push(2);

        writer.flush().unwrap();
    }
}
