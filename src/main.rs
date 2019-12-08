use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
struct Planet {
    id: String,
    parent: Box<Planet>,
    planets_orbiting: Vec<Box<Planet>>,
}

impl Planet {
    pub fn new(data: String, parent: Box<Planet>) -> Planet {
        Planet {
            id: data,
            parent: parent,
            planets_orbiting: Vec::new(),
        }
    }

    fn add_planets_orbiting(&mut self, planet: Planet) {
        self.planets_orbiting.push(Box::new(planet));
    }

    fn count(&self, level: u32) -> u32 {
        let mut counter: u32 = 0;

        for planet in self.planets_orbiting.iter() {
            counter += planet.count(level + 1);
        }
        counter + level
    }

    fn insert(&mut self, new_planet: Planet, dest_planet: Box<Planet>) -> bool {
        let mut result = false;
        if self.id == dest_planet.id {
            self.add_planets_orbiting(new_planet);
            result = true
        } else {
            for planet in self.planets_as_mut() {
                if result == false {
                    result = planet.insert(new_planet.clone(), dest_planet.clone());
                }
            }
        }
        result
    }

    fn planets_as_mut(&mut self) -> &mut Vec<Box<Planet>> {
        &mut self.planets_orbiting
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut com = Planet::new("COM".to_string());

    let mut input: Vec<String> = Vec::new();

    for line in reader.lines() {
        input.push(line?);
    }

    while input.len() > 0 {
        input.retain(|data| {
            let planets: Vec<String> = data.split(')').map(|s| s.to_string()).collect();
            !com.insert(
                Planet::new(planets[1].clone()),
                Box::new(Planet::new(planets[0].clone())),
            )
        });
    }
    println!("COM: {:?}", com.count(0));

    Ok(())
}
