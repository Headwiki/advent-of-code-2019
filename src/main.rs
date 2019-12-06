use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
struct Planet {
    id: String,
    planets_orbiting: Vec<Box<Planet>>
}

impl Planet {
    pub fn new(data: String) -> Planet {
        Planet {
            id: data,
            planets_orbiting: Vec::new()
        }
    }

    fn add_planets_orbiting(&mut self, planet: Planet) {
        self.planets_orbiting.push(Box::new(planet));
    }

    fn count(&mut self, level: u32) -> u32 {

        let mut counter: u32 = 0;
        println!("Planet: {} start counter {}",self.id, counter);

        println!("Planet: {}, Orbiting planets: {:?}", self.id, self.planets_orbiting);
        for planet in self.planets_orbiting.clone() {
            counter += planet.clone().count(level+1);
        }
        println!("Planet: {} end counter {}",self.id, counter);
        counter + level
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    let mut com_planet = Planet::new("COM".to_string());
    let mut b_planet = Planet::new("B".to_string());
    let c_planet = Planet::new("C".to_string());
    let d_planet = Planet::new("D".to_string());
    let e_planet = Planet::new("E".to_string());

    com_planet.add_planets_orbiting(b_planet);
    com_planet.add_planets_orbiting(c_planet);
    com_planet.planets_orbiting[0].add_planets_orbiting(d_planet);
    com_planet.planets_orbiting[0].add_planets_orbiting(e_planet);

    println!("COM: {:?}", com_planet.count(0));

    Ok(())

}
