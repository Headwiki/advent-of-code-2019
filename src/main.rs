use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let valid_numbers = find_valid_numbers(&line?);
        println!("{:?}", find_valid_numbers_again(&valid_numbers).len());
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

fn find_valid_numbers_again(numbers: &Vec<u32>) -> Vec<u32> {

    let mut valid_numbers: Vec<u32> = Vec::new();

    for number in numbers {
        let stringed_number = number.to_string();
        let mut frequency: HashMap<String, u32> = HashMap::new();

        for c in stringed_number.chars() {
            *frequency.entry(c.to_string()).or_insert(0) += 1;
        }
        let mut count_frequency: Vec<(&String, &u32)> = frequency.iter().collect();
        count_frequency.sort_by(|a, b| b.1.cmp(a.1));

        if count_frequency.len() > 0 {
            if *count_frequency[0].1 == 2 {
                valid_numbers.push(*number);
            }
        }

        if count_frequency.len() > 1 {
            if *count_frequency[1].1 == 2 && (*count_frequency[0].1 > *count_frequency[1].1) {
                valid_numbers.push(*number);
            }
        }

    }
    valid_numbers
}