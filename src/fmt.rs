// Output utils

use std::fmt;

pub struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        print!("{}", s);
        Ok(())
    }
    fn write_char(&mut self, s: char) -> Result<(), fmt::Error> {
        print!("{}", s);
        Ok(())
    }
    fn write_fmt(&mut self, s: fmt::Arguments) -> Result<(), fmt::Error> {
        print!("{}", s);
        Ok(())
    }
}
