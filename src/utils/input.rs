use std::io::{self, Write};

use log::warn;

pub fn input(message: &str) -> Result<String, io::Error> {
    let mut input = String::new();

    while input.len() <= 1 {
        print!("{} ", message);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read password");

        if input.len() <= 1 {
            dbg!(input.len());
            warn!("Please fill in the fields");
            input = String::new();
        }
    }

    input.pop(); // INFO: Delete the '\n' at the end

    Ok(input)
}
