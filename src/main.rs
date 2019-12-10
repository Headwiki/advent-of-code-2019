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

    for line in reader.lines() {
        input.push(line?.as_bytes().to_vec());
    }

    println!("{:?}", input);

    /* for (i, row) in input.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            // If asteroid
            if *col == 35 {
                ping((i as u8, j as u8), &mut asteroids);
            }
        }
    } */

    // test
    scan_line((0, 1), (6, 4));

    Ok(())
}

fn scan_line(start: (i8, i8), end: (i8, i8)) {
    let mut first = start;
    let second = end;

    let dx = (second.0 - first.0).abs();
    let dy = (second.1 - first.1).abs();
    let sx = if first.0 < second.0 { 1 } else { -1 };
    let sy = if first.1 < second.1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        println!("x: {}, y: {} ", first.0, first.1);
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
}