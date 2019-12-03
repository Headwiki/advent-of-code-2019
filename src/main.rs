use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Intersection {
    point: Point,
    len: u32
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

    for point_a in paths[0].iter() {
        for point_b in paths[1].iter() {
            if (point_a == point_b) && (point_a != &Point {x: 0, y: 0}) {
                println!("{:?}", point_a);
                intersections.push(point_a.clone());
            }
        }
    }

    /* for point in intersections {
        let sum = point.x.abs() + point.y.abs();
        if sum < min {
            min = sum;
        }
    } */

    // Part 2

    let mut shortest_intersection = Intersection { point: Point {x: 0, y: 0}, len: core::u32::MAX };

    for intersection in intersections.iter() {
        let mut path_one_length = 0;
        for point in paths[0].iter() {
            if point == intersection {
                println!("Break at intersection {:?}, path 0: {:?}", &intersection, &path_one_length);
                break;
            }
            path_one_length += 1;
        }

        let mut path_two_length = 0;
        for point in paths[1].iter() {
            if point == intersection {
                println!("Break at intersection {:?}, path 1: {:?}", &intersection, &path_two_length);
                break;
            }
            path_two_length += 1;
        }

        let sum = path_one_length + path_two_length;
        if sum < shortest_intersection.len {
            shortest_intersection.point = intersection.clone();
            shortest_intersection.len = sum;
        }

    }

    println!("{:?}", shortest_intersection);

    Ok(())
}

fn parse_path(path: &String) -> Vec<Point> {
    let directions: Vec<&str> = path.split(',').collect();
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