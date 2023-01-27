#[macro_export]
#[allow(unused_macros)]

macro_rules! write_vec {
    ($writer:ident, $values:expr) => {
        write_vec!($writer, $values, sep: ' ');
    };

    ($writer:ident, $values:expr,sep: $sep:expr) => {
        let n = $values.len();
        if n == 0 {
            writeln!($writer).unwrap();
            return;
        }
        for i in 0..n - 1 {
            write!(
                $writer,
                "{}{}",
                $values[i], $sep
            )
            .unwrap();
        }
        writeln!($writer, "{}", $values[n - 1]).unwrap();
    };
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
