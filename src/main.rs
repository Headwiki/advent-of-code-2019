use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut int_codes: Vec<u32> = Vec::new();

    for line in reader.lines() {
        int_codes = line.unwrap().split(',').map(|i| i.parse::<u32>().unwrap()).collect();
    }

    println!("{:?}", int_codes);

    Ok(())
}

fn execute_intcode (intcodes: Vec<u32>) -> Vec<u32> {
    unimplemented!();
}