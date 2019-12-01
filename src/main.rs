use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        sum += total_fuel(line?.parse::<u32>().unwrap())
    }

    println!("{}", sum);
    Ok(())
}

fn fuel_required(mass: u32) -> u32 {
    (mass as f32 / 3.0).floor() as u32 - 2
}

fn total_fuel(mut mass: u32) -> u32{
    let mut total = 0;

    while mass > 5 {
        println!("Total: {}, Mass: {}", total, mass);
        mass = fuel_required(mass);
        total += mass;
    }

    total
}