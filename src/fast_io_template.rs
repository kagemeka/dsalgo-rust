//! mainly used in competitive programming
//! template main function.
#![allow(unused_imports)]

pub use crate::{
    io_locked_stdin_reader::*,
    io_locked_stdout_buffered_writer::*,
};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     use std::io::Write;
//     let mut reader = locked_stdin_reader();
//     let mut writer = locked_stdout_buf_writer();
//     writer.flush()?;
//     Ok(())
// }
