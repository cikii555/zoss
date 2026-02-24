use std::fs;

fn secure_file_access(filename: &str) -> Result<(), std::io::Error> {

    let contents = fs::read_to_string(filename)?;

    println!("Bank safely read:\n{}", contents);

    Ok(())
}

fn main() {

    let filename = "../data/account.txt";

    println!("SECURE BANK SERVICE");

    if let Err(err) = secure_file_access(filename) {
        eprintln!("Error: {}", err);
    }
}
