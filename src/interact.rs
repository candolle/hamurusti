use std::{error::Error, io};
use textwrap::{fill, termwidth};

/// Print to the width of the terminal.
pub fn print_width(text: &str) {
    println!("{}", fill(text, termwidth()));
}

/// Read a non-negative number from the terminal. Non-digit characters will be
/// stripped out, so for example "1,200" is a valid input, in case the player
/// matches the format of numbers that are printed. If the non-digit-stripped
/// input is not a valid non-negative number, Hamurusti will complain and the
/// player will be given the chance to try again.
pub fn read_number() -> Result<u32, Box<dyn Error>> {
    loop {
        let mut entry = String::new();
        io::stdin().read_line(&mut entry)?;
        entry.retain(|c| c.is_digit(10));
        match entry.parse::<u32>() {
            Err(_) => print_width("\nHamurusti: I cannot do what you wish. \
                Give me a sensible answer, or get yourself another steward!"),
            Ok(number) => return Ok(number),
        }
    }
}
