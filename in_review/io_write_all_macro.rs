#[macro_export]
#[allow(unused_macros)]

macro_rules! write_all {
    ($writer:ident) => {
        writeln!($writer).unwrap();
    };

    ($writer:ident, $v:expr) => {
        writeln!($writer, "{}", $v).unwrap();
    };

    ($writer:ident, $v:expr, $($values:expr),+) => {
        write!($writer, "{} ", $v).unwrap();
        write_all!($writer, $($values),*);
    };
}

#[cfg(test)]

mod tests {

    #[test]

    fn write_macro() {
        use std::io::Write;

        use crate::io_locked_stdout_buffered_writer::*;

        let mut writer = locked_stdout_buf_writer();

        let mut v = vec![];

        v.push(1);

        v.push(2);

        write_all!(writer);

        write_all!(writer, 1, 2, 3);

        write_all!(writer, 1);

        writer.flush().unwrap();
    }
}
