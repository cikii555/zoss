use std::fs;
use std::thread;
use std::time::Duration;

fn insecure_file_access(filename: &str) -> Result<(), std::io::Error> {

    
    if fs::metadata(filename).is_ok() {

        println!("Bank verified account file exists");

        
        thread::sleep(Duration::from_millis(300));

        
        let contents = fs::read_to_string(filename)?;

        println!("BANK READ:\n{}", contents);

        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Account file not found",
        ))
    }
}

fn main() {

    let filename = "../data/account.txt";

    println!("VULNERABLE BANK SERVICE");

    if let Err(err) = insecure_file_access(filename) {
        eprintln!("Error: {}", err);
    }
}