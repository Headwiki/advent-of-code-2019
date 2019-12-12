use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vel_x: i32,
    vel_y: i32,
    vel_z: i32
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Moon {
            x: x,
            y: y,
            z: z,
            vel_x: 0,
            vel_y: 0,
            vel_z: 0,
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut moons: Vec<Moon> = Vec::new();

    for line in reader.lines() {
        let mut string = line?;
        string = string.to_string().clone();
        let input_data: Vec<_> = string[1..string.len() -1].split(',').collect();
        let mut moon_data: Vec<Vec<&str>> = Vec::new();

        for data in input_data.iter() {
            moon_data.push(data.split('=').collect());
        }
        moons.push(Moon::new(
            moon_data[0][1].parse::<i32>().unwrap(), 
            moon_data[1][1].parse::<i32>().unwrap(), 
            moon_data[2][1].parse::<i32>().unwrap()));

    }
        println!("{:?}", moons);

    Ok(())
}
