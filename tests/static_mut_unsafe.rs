#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        static mut M: i64 = 1;

        unsafe { M = 0 };

        unsafe { dbg!(M) };
    }
}
