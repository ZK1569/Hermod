use std::io;

pub fn input(message: &str) -> Result<String, io::Error> {
    println!("{}", message);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read password");

    input.pop(); // INFO: Delete the '\n' at the end

    Ok(input)
}
