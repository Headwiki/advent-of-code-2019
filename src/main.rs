use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", find_valid_numbers(&line?).len());
    }

    Ok(())
}

fn find_valid_numbers(range: &String) -> Vec<u32> {
    let range: Vec<u32> = range.split('-').map(|x| x.parse::<u32>().unwrap()).collect();
    let mut valid_numbers: Vec<u32> = Vec::new();

    for x in range[0]..range[1] + 1 {
        if is_valid_number(x) {
            valid_numbers.push(x);
        }
    }

    valid_numbers
}

fn is_valid_number(number: u32) -> bool {
    let mut flag_double = false;
    let mut flag_never_decrease = true;
    const RADIX: u32 = 10;

    let stringed_number = number.to_string();
    let sub_numbers: Vec<u32> = stringed_number.chars().map(|x| x.to_digit(RADIX).unwrap()).collect();

    for (i, x) in sub_numbers.iter().enumerate() {
        if i < sub_numbers.len()-1 {
            if *x == sub_numbers[i+1] {
                flag_double = true;
                break;
            }
        }
    }

    for (i, x) in sub_numbers.iter().enumerate() {
        if i < sub_numbers.len()-1 {
            if *x > sub_numbers[i+1] {
                flag_never_decrease = false;
                break;
            }
        }
    }

    flag_double && flag_never_decrease
}