use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
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

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x 
        && self.y == other.y
        && self.z == other.z
        && self.vel_x == other.vel_x
        && self.vel_y == other.vel_y
        && self.vel_y == other.vel_y
    }
}

struct Moons {
    moons: Vec<Moon>
}

impl Moons {
    fn step(&mut self, steps: u32) {
        for step in 0..steps {
            let mut moons: Vec<Moon> = Vec::new();
            for i in 0..self.moons.len() {
                let mut moon = self.moons[i].clone();

                for other_moon in self.moons.iter() {
                    if self.moons[i] != *other_moon {
                        if self.moons[i].x < other_moon.x {
                            moon.x += 1;
                        } else if self.moons[i].x > other_moon.x {
                            moon.x -= 1;
                        }

                        if self.moons[i].y < other_moon.y {
                            moon.y += 1;
                        } else if self.moons[i].y > other_moon.y {
                            moon.y -= 1;
                        }

                        if self.moons[i].z < other_moon.z {
                            moon.z += 1;
                        } else if self.moons[i].z > other_moon.z {
                            moon.z -= 1;
                        }
                    }
                }


                // Add old velocity
                moon.x += self.moons[i].vel_x;
                moon.y += self.moons[i].vel_y;
                moon.z += self.moons[i].vel_z;
                
                // Calculate new velocity
                println!("self vel x: {}, new vel x: {}", self.moons[i].x, moon.vel_x);
                moon.vel_x = diff(self.moons[i].x, moon.x);
                println!("New Vel X: {}", moon.vel_x);
                println!("self vel y: {}, new vel y: {}", self.moons[i].y, moon.vel_y);
                moon.vel_y = diff(self.moons[i].y, moon.y);
                println!("New Vel Y: {}", moon.vel_y);
                println!("self vel z: {}, new vel z: {}", self.moons[i].z, moon.vel_z);
                moon.vel_z = diff(self.moons[i].z, moon.z);
                println!("New Vel Z: {}", moon.vel_z);

                // Set updated data
                moons.push(moon);
            }
            self.moons = moons
        }
    }
}

fn diff(num1: i32, num2: i32) -> i32 {
    // if both nums are positive or negative
    println!("Got num1: {}, num2: {}", num1, num2);

    if num1 > 0 {
        if num2 < 0 {
            return num1 - num2
        } else {
            return num2 - num1
        }
    } else {
        return num2 - num1
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut moons: Moons = Moons { moons: Vec::new() };

    for line in reader.lines() {
        let mut string = line?;
        string = string.to_string().clone();
        let input_data: Vec<_> = string[1..string.len() -1].split(',').collect();
        let mut moon_data: Vec<Vec<&str>> = Vec::new();

        for data in input_data.iter() {
            moon_data.push(data.split('=').collect());
        }
        moons.moons.push(Moon::new(
            moon_data[0][1].parse::<i32>().unwrap(), 
            moon_data[1][1].parse::<i32>().unwrap(), 
            moon_data[2][1].parse::<i32>().unwrap()));

    }
        println!("{:?}", moons.moons);
        moons.step(1);
        println!("{:?}", moons.moons);
        moons.step(1);
        println!("{:?}", moons.moons);


        

    Ok(())
}
