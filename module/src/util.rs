use std::fmt::Display;
use std::io::{stderr, Write};
use std::process;

pub fn exit<T: Display, E: Display>(code: i32, title: T, error: E) -> ! {
    writeln!(&mut stderr(), "{}: {}", title, error).unwrap();
    process::exit(code)
}
