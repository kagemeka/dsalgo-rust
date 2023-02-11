/// reference
/// https://users.rust-lang.org/t/show-value-only-in-debug-mode/43686/3
#[macro_export]

// #[allow(unused_macros)]
macro_rules! dbg {
    ($($x:tt)*) => {
        {
            // default in debug mode
            #[cfg(debug_assertions)]
            {
                std::dbg!($($x)*)
            }

            // default in release mode
            #[cfg(not(debug_assertions))]
            {
                ($($x)*)
            }
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test_dbg() {
        let a = 1;

        dbg!(dbg!(a) + 1);
    }
}
