use std::io;
use benchmark::cpu;
mod benchmark;

fn main() {
    loop {
    println!("\n1 - CPU, 2 - RAM, 3 - disk");
    let mut operation = String::new();
    if let Err(err) = io::stdin().read_line(&mut operation) {
        println!("{err}");
    };
    let operation = match operation.trim().parse::<u8>() {
        Ok(num) => num,
        Err(err) => {
            println!("\n{err}: Please select a valid option");
            continue;
        },
    };

    match operation {
        1 => {
            cpu::single_start();
        }, 2 => {
        }, 3 => {
        }, _ => {
            println!("\nPlease select a valid option");
            continue;
        }
    };
    break;
    };

}
