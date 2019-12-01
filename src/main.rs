use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        sum += fuel_required(line?.parse::<f32>().unwrap())
    }

    println!("{}", sum);
    Ok(())
}

fn fuel_required(mass: f32) -> u32 {
    ((mass / 3.0).floor() - 2.0) as u32
}
