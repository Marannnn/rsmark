use std::io;
use benchmark::cpu;
mod benchmark;

fn main() {
    println!("1 - CPU, 2 - RAM, 3 - disk");
    let mut operation = String::new();
    if let Err(err) = io::stdin().read_line(&mut operation) {
        println!("{err}");
    };
    let operation = match operation.trim().parse::<u8>() {
        Ok(num) => num,
        Err(err) => {
            println!("{err}");
            panic!();
        },
    };

    match operation {
        1 => {
            cpu::single_start();
        }, 2 => {
        }, 3 => {
        }, _ => {
        }
    };
}
