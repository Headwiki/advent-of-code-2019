use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut intcodes: Vec<u32> = Vec::new();

    for line in reader.lines() {
        intcodes = line.unwrap().split(',').map(|i| i.parse::<u32>().unwrap()).collect();
    }

    println!("{:?}", execute_intcode(intcodes));

    Ok(())
}

fn execute_intcode (mut intcodes: Vec<u32>) -> Vec<u32> {
    let mut intcode_op_pos = 0;

    // While operation code not eqult to 99 - 'halt'
    loop{
        match intcodes[intcode_op_pos] {
            1 => {
                let first_pos = intcodes[intcode_op_pos+1] as usize;
                let second_pos = intcodes[intcode_op_pos+2] as usize;
                let result_pos = intcodes[intcode_op_pos+3] as usize;

                intcodes[result_pos] = intcodes[first_pos] + intcodes[second_pos];
            },
            2 => {
                let first_pos = intcodes[intcode_op_pos+1] as usize;
                let second_pos = intcodes[intcode_op_pos+2] as usize;
                let result_pos = intcodes[intcode_op_pos+3] as usize;

                intcodes[result_pos] = intcodes[first_pos] * intcodes[second_pos];
            },
            99 => { return intcodes },
            _ => { panic!("Unknown operation code!"); }
        }
        intcode_op_pos += 4;
    }

}