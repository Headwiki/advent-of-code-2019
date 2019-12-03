use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Point {
    x: i32,
    y: i32
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut paths: Vec<Vec<Point>> = Vec::new();
    let mut intersections: Vec<Point> = Vec::new();

    let mut min = core::i32::MAX;

    for line in reader.lines() {
        paths.push(parse_path(&line?));
    }

    for point_a in paths[0].clone() {
        for point_b in paths[1].clone() {
            if ((point_a == point_b) && (point_a != Point {x: 0, y: 0})) {
                println!("{:?}", point_a);
                intersections.push(point_a.clone());
            }
        }
    }

    for point in intersections {
        let sum = point.x.abs() + point.y.abs();
        if sum < min {
            min = sum;
        }
    }

    println!("{:?}", min);

    Ok(())
}

fn parse_path(path: &String) -> Vec<Point> {
    let mut directions: Vec<&str> = path.split(',').collect();
    let mut path: Vec<Point> = Vec::new();

    path.push(Point {x: 0, y: 0});

    for direction in directions {
        path.append(&mut parse_direction(&path.last().unwrap(), &direction));
    }

    path
}

fn parse_path_intersection(path: &String) -> Vec<Point> {
    let mut directions: Vec<&str> = path.split(',').collect();
    let mut path: Vec<Point> = Vec::new();

    path.push(Point {x: 0, y: 0});

    for direction in directions {
        path.append(&mut parse_direction(&path.last().unwrap(), &direction));
    }

    path
}

fn parse_direction(start_point: &Point, direction: &str) -> Vec<Point> {
    let mut directions: Vec<Point> = Vec::new();

    match direction.chars().next().unwrap() {
        'R' => {
            for i in 1..direction.get(1..).unwrap().parse::<i32>().unwrap()+1 {
                directions.push(
                    Point { 
                        x: start_point.x+i, 
                        y: start_point.y 
                    }
                );
            }
        },
        'D' => {
            for i in 1..direction.get(1..).unwrap().parse::<i32>().unwrap()+1 {
                directions.push(
                    Point { 
                        x: start_point.x, 
                        y: start_point.y-i 
                    }
                );
            }
        },
        'L' => {
            for i in 1..direction.get(1..).unwrap().parse::<i32>().unwrap()+1 {
                directions.push(
                    Point { 
                        x: start_point.x-i, 
                        y: start_point.y 
                    }
                );
            }
        },
        'U' => {
            for i in 1..direction.get(1..).unwrap().parse::<i32>().unwrap()+1 {
                directions.push(
                    Point { 
                        x: start_point.x, 
                        y: start_point.y+i 
                    }
                );
            }
        },
        _ => { panic!("Unexpected character in direction!"); },
    }

    directions
}