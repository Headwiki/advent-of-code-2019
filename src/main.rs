use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut intcode: Vec<i32> = Vec::new();

    for line in reader.lines() {
        intcode = line.unwrap().split(',').map(|i| i.parse::<i32>().unwrap()).collect();
    }

    execute_intcode(intcode);
    //println!("{:?}", brute_intcodes(&intcodes));
    Ok(())
}

fn execute_intcode (mut intcode: Vec<i32>) -> Vec<i32> {
    let mut instruction_position = 0;

    loop{
        let mut instruction = intcode[instruction_position];
        let mut jumped = false;

        // parse instruction
        let instruction_vec: Vec<u32> = 
            instruction.to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect();
        
        let instruction_length = instruction_vec.len();
        
        let operation_mode = instruction % 100;

        let mut parameter_mode_one = 0;
        let mut parameter_mode_two = 0;

        if instruction_length == 4 {
            parameter_mode_two = instruction_vec[0];
            parameter_mode_one = instruction_vec[1];

        } else if instruction_length == 3 {
            parameter_mode_one = instruction_vec[0];
        }

        // Match operation mode
        match operation_mode {
            1 => {
                // add 
                let mut first_pos = 0;
                let mut second_pos = 0;
                let result_pos = intcode[instruction_position+3] as usize;

                if parameter_mode_one == 0 {
                    first_pos = intcode[intcode[instruction_position+1] as usize];
                } else {
                    first_pos = intcode[instruction_position+1];
                }

                if parameter_mode_two == 0 {
                    second_pos = intcode[intcode[instruction_position+2] as usize];
                } else {
                    second_pos = intcode[instruction_position+2];
                }
                intcode[result_pos] = first_pos + second_pos;
            },
            2 => {
                // multiply
                let mut first_pos = 0;
                let mut second_pos = 0;
                let result_pos = intcode[instruction_position+3] as usize;

                if parameter_mode_one == 0 {
                    first_pos = intcode[intcode[instruction_position+1] as usize];
                } else {
                    first_pos = intcode[instruction_position+1];
                }

                if parameter_mode_two == 0 {
                    second_pos = intcode[intcode[instruction_position+2] as usize];
                } else {
                    second_pos = intcode[instruction_position+2];
                }

                intcode[result_pos] = first_pos * second_pos;
            },
            3 => {
                // input
                let first_pos = intcode[instruction_position+1] as usize;
                let mut input = String::new();
                println!("Enter input: ");

                std::io::stdin().lock().read_line(&mut input).unwrap();
                input.pop();    // remove trailing newline
                input.pop();
                intcode[first_pos] = input.parse::<i32>().unwrap();
            },
            4 => {
                //output
                let mut first_pos = 0;

                if parameter_mode_one == 0 {
                    first_pos = intcode[intcode[instruction_position+1] as usize];
                } else {
                    first_pos = intcode[instruction_position+1];
                }
                println!("Output: {}", first_pos);
            },
            5 => {
                // jump-if-true
                let mut first_pos = 0;
                let mut second_pos = 0;

                if parameter_mode_one == 0 {
                    first_pos = intcode[intcode[instruction_position+1] as usize];
                } else {
                    first_pos = intcode[instruction_position+1];
                }

                if first_pos != 0 {
                    if parameter_mode_two == 0 {
                        second_pos = intcode[intcode[instruction_position+2] as usize];
                    } else {
                        second_pos = intcode[instruction_position+2];
                    }
                    instruction_position = second_pos as usize;
                    jumped = true;
                }
            },
            6 => {
                // jump-if-false
                let mut first_pos = 0;
                let mut second_pos = 0;

                if parameter_mode_one == 0 {
                    first_pos = intcode[intcode[instruction_position+1] as usize];
                } else {
                    first_pos = intcode[instruction_position+1];
                }
                if first_pos == 0 {
                    if parameter_mode_two == 0 {
                        second_pos = intcode[intcode[instruction_position+2] as usize];
                    } else {
                        second_pos = intcode[instruction_position+2];
                    }
                    instruction_position = second_pos as usize;
                    jumped = true;
                }
            },
            7 => {
                // less-than
                let mut first_pos = 0;
                let mut second_pos = 0;
                let result_pos = intcode[instruction_position+3] as usize;

                if parameter_mode_one == 0 {
                    first_pos = intcode[intcode[instruction_position+1] as usize];
                } else {
                    first_pos = intcode[instruction_position+1];
                }

                if parameter_mode_two == 0 {
                    second_pos = intcode[intcode[instruction_position+2] as usize];
                } else {
                    second_pos = intcode[instruction_position+2];
                }

                if first_pos < second_pos {
                    intcode[result_pos] = 1;
                } else {
                    intcode[result_pos] = 0;
                }
            },
            8 => {
                // equal
                let mut first_pos = 0;
                let mut second_pos = 0;
                let result_pos = intcode[instruction_position+3] as usize;

                if parameter_mode_one == 0 {
                    first_pos = intcode[intcode[instruction_position+1] as usize];
                } else {
                    first_pos = intcode[instruction_position+1];
                }

                if parameter_mode_two == 0 {
                    second_pos = intcode[intcode[instruction_position+2] as usize];
                } else {
                    second_pos = intcode[instruction_position+2];
                }

                if first_pos == second_pos {
                    intcode[result_pos] = 1;
                } else {
                    intcode[result_pos] = 0;
                }
            },
            99 => { return intcode },
            _ => { panic!("Unknown operation code!"); }
        }

        if !jumped {
            if (operation_mode == 3) ||(operation_mode == 4) {
                instruction_position += 2;
            } else if (operation_mode == 5) || (operation_mode == 6) {
                instruction_position += 3;
            } else {
                instruction_position += 4;
            }
        }
    }

}

/* fn brute_intcodes(intcode: &Vec<i32>) -> Vec<i32> {
    let mut intcode = intcode.clone();
    for x in 0..100 {
        intcode[1] = x;
        for y in 0..100 {
            intcode[2] = y;

            let result = execute_intcode(intcode.clone());
            if result[0] == 19690720 {
                return result
            }
        }
    }
    vec![0]
} */