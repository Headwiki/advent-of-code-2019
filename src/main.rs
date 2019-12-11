use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut input: Vec<Vec<u8>> = Vec::new();

    // Hashmap with coordinate as key, value is a vec with surrounding asteroids
    let mut asteroids: HashMap<(u8, u8), Vec<(u8, u8)>> = HashMap::new();
    
    // ((coordinate), count)
    let mut max_asteroid: ((u8, u8), u8) = ((0, 0), 0);

    // Load input data
    for line in reader.lines() {
        input.push(line?.as_bytes().to_vec());
    }

    // All coords to scan towards
    let border_coords: Vec<(u8, u8)> = create_border_coords(input.len() as u8, input[0].len() as u8);
    
    // Find all asteroids in input
    for (j, row) in input.iter().enumerate() {
        for (i, col) in row.iter().enumerate() {
            // If asteroid
            if *col == 35 {
                println!("Found asteroid: {:?}", (i as u8, j as u8));
                // For each coordinate in border
                for coord in border_coords.iter() {
                    let targeted_coords = scan_line((i as i8, j as i8), (coord.0 as i8, coord.1 as i8));
                    // For each coordinate in path from a to b
                    for targeted_coord in targeted_coords.iter() {
                        if *targeted_coord != (i as u8, j as u8) {
                            // If asteroid
                            let possible_asteroid = input[targeted_coord.1 as usize][targeted_coord.0 as usize];
                            if possible_asteroid == 35 {
                                println!("{:?} sees {:?}", (i as u8, j as u8), targeted_coord);
                                let is_present = asteroids
                                    .entry((i as u8, j as u8))
                                    .or_insert(Vec::new())
                                    .iter()
                                    .any(|c| c == targeted_coord);

                                if !is_present {
                                    asteroids.entry((i as u8, j as u8))
                                    .or_insert(Vec::new())
                                    .push(*targeted_coord)
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    // test
    //println!("{:?}", scan_line((0, 1), (6, 4)));
    println!("{:?}", asteroids);

    Ok(())
}

// Returns vector containing all coordinates from point a to b
fn scan_line(start: (i8, i8), end: (i8, i8)) -> Vec<(u8, u8)> {
    let mut first = start;
    let second = end;
    let mut result = Vec::new();

    let dx = (second.0 - first.0).abs();
    let dy = (second.1 - first.1).abs();
    let sx = if first.0 < second.0 { 1 } else { -1 };
    let sy = if first.1 < second.1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        result.push((first.0 as u8, first.1 as u8));
        if (first.0 == second.0) && (first.1 == second.1) {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            first.0 += sx;
        }
        if e2 < dx {
            err += dx;
            first.1 += sy;
        }
    }
    result
}

fn create_border_coords(rows: u8, cols: u8) -> Vec<(u8, u8)> {

    let mut coords = Vec::new();

    // Top row
    for x in 0..cols {
        coords.push((x as u8, 0));
    }

    // Right column
    for y in 1..rows {
        coords.push(((rows-1) as u8, y as u8));
    }

    // Bottom row
    for x in (0..cols-1).rev() {
        coords.push((x as u8, (cols-1) as u8));
    }

    // Left column
    for y in (1..rows-1).rev() {
        coords.push((0, y as u8));
    }
    
    coords
}