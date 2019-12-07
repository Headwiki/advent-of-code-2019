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

    fn count(&self, level: u32) -> u32 {

        let mut counter: u32 = 0;
        println!("Planet: {} start counter {}", self.id, counter);

        println!("Planet: {}, Orbiting planets: {:?}", self.id, self.planets_orbiting);
        for planet in self.planets_orbiting.iter() {
            counter += planet.count(level+1);
        }
        println!("Planet: {} end counter {}", self.id, counter);
        counter + level
    }

    fn insert(&mut self, new_planet: Planet, dest_planet: Box<Planet>) -> Box<Planet> {
        if self.id == dest_planet.id {
            println!("Found {:?} in {:?}", dest_planet, self.id);
            self.add_planets_orbiting(new_planet);
        } else {
            for planet in self.planets_as_mut() {
                planet.insert(new_planet.clone(), dest_planet.clone());
            }
        }
        dest_planet
    }

    fn planets_as_mut(&mut self) -> &mut Vec<Box<Planet>> {
        &mut self.planets_orbiting
    }

}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut first_planet_is_set = false;
    let mut first_planet = Planet::new("0".to_string());

     for line in reader.lines() {
        if !first_planet_is_set {
            let planets: Vec<String> = line.unwrap().split(')').map(|s| s.to_string()).collect();
            first_planet = Planet::new(planets[0].clone());
            first_planet.add_planets_orbiting(Planet::new(planets[1].clone()));
            first_planet_is_set = true;
        } else {
            let planets: Vec<String> = line.unwrap().split(')').map(|s| s.to_string()).collect();
            // find planet planets[0] and insert planets[1]
            first_planet.insert(Planet::new(planets[1].clone()), Box::new(Planet::new(planets[0].clone())));
        }
    } 

    println!("COM: {:?}", first_planet.count(0));
/*     let mut com_planet = Planet::new("COM".to_string());
    let mut b_planet = Planet::new("B".to_string());
    let c_planet = Planet::new("C".to_string());
    let d_planet = Planet::new("D".to_string());
    let e_planet = Planet::new("E".to_string());

    com_planet.add_planets_orbiting(b_planet);
    com_planet.add_planets_orbiting(c_planet);
    com_planet.planets_orbiting[0].add_planets_orbiting(d_planet);
    com_planet.planets_orbiting[0].add_planets_orbiting(e_planet);  




    println!("COM: {:?}", com_planet.count(0)); */

    Ok(())

}
