use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut intcode = Intcode::new();

    for line in reader.lines() {
        intcode.data = line
            .unwrap()
            .split(',')
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
    }

    intcode.parse();

    Ok(())
}

struct Intcode {
    data: Vec<i32>,
    instruction_pointer: u32,
    current_instruction: Instruction,
    jumped: bool,
}

impl Intcode {
    pub fn new() -> Self {
        Intcode {
            data: Vec::new(),
            instruction_pointer: 0,
            current_instruction: Instruction {
                first_parameter: 0,
                second_parameter: 0,
                third_parameter: 0,
                operation_mode: 0,
            },
            jumped: false,
        }
    }

    fn update_jumped(&mut self) {
        if !self.jumped {
            if (self.current_instruction.operation_mode == 3) || (self.current_instruction.operation_mode == 4) {
                self.instruction_pointer += 2;
            } else if (self.current_instruction.operation_mode == 5) || (self.current_instruction.operation_mode == 6) {
                self.instruction_pointer += 3;
            } else {
                self.instruction_pointer += 4;
            }
        }
    }

    fn parse(&mut self) {
        loop {

            // Set new instruction
            self.current_instruction =
                Instruction::new(
                    self.data[self.instruction_pointer as usize] as u32
                );

            // Clear 'jumped'
            self.jumped = false;

            // Match operation mode
            match self.current_instruction.operation_mode {
                1 => {
                    // Add 
                    let result_pos = self.data[(self.instruction_pointer + 3 ) as usize] as usize;
                    self.data[result_pos] = self.current_instruction.get_parameter_value(1, &self.data, self.instruction_pointer) 
                        + self.current_instruction.get_parameter_value(2, &self.data, self.instruction_pointer);
                },
                2 => {
                    // Multiply
                    let result_pos = self.data[(self.instruction_pointer + 3 ) as usize] as usize;
                    self.data[result_pos] = self.current_instruction.get_parameter_value(1, &self.data, self.instruction_pointer) 
                        * self.current_instruction.get_parameter_value(2, &self.data, self.instruction_pointer);
                },
                3 => {
                    // Read input
                    let first_pos = self.data[(self.instruction_pointer + 1) as usize] as usize;
                    let mut input = String::new();
                    println!("Enter input: ");

                    std::io::stdin().lock().read_line(&mut input).unwrap();
                    input.pop(); // remove trailing newline
                    input.pop();
                    self.data[first_pos] = input.parse::<i32>().unwrap();
                },
                4 => {
                    // Print output
                    println!("Output: {}", self.current_instruction.get_parameter_value(1, &self.data, self.instruction_pointer));
                },
                5 => {
                    // Jump-if-true
                    if self.current_instruction.get_parameter_value(1, &self.data, self.instruction_pointer) != 0 {
                        self.instruction_pointer = self.current_instruction.get_parameter_value(2, &self.data, self.instruction_pointer) as u32;
                        self.jumped = true;
                    }
                },
                6 => {
                    // Jump-if-false
                    if self.current_instruction.get_parameter_value(1, &self.data, self.instruction_pointer) == 0 {
                        self.instruction_pointer = (self.current_instruction.get_parameter_value(2, &self.data, self.instruction_pointer)) as u32;
                        self.jumped = true;
                    }
                },
                7 => {
                    // Less-than
                    let result_pos = self.data[(self.instruction_pointer + 3) as usize] as usize;

                    if self.current_instruction.get_parameter_value(1, &self.data, self.instruction_pointer) 
                        < self.current_instruction.get_parameter_value(2, &self.data, self.instruction_pointer) {
                        self.data[result_pos] = 1;
                    } else {
                        self.data[result_pos] = 0;
                    }
                },
                8 => {
                    // Equal
                    let result_pos = self.data[(self.instruction_pointer + 3) as usize] as usize;

                    if self.current_instruction.get_parameter_value(1, &self.data, self.instruction_pointer) 
                        == self.current_instruction.get_parameter_value(2, &self.data, self.instruction_pointer) {
                        self.data[result_pos] = 1;
                    } else {
                        self.data[result_pos] = 0;
                    }
                },
                99 => return,
                _ => {
                    panic!("Unknown operation code!");
                }
            }

            self.update_jumped();
        }
    }
}

struct Instruction {
    first_parameter: u32,
    second_parameter: u32,
    third_parameter: u32,
    operation_mode: u32,
}

impl Instruction {
    pub fn new(number: u32) -> Self {
        let vec = number_to_vec(number);
        let mut parameter_mode_one = 0;
        let mut parameter_mode_two = 0;
        let mut parameter_mode_three = 0;

        if vec.len() == 5 {
            parameter_mode_three = vec[0];
            parameter_mode_two = vec[1];
            parameter_mode_one = vec[2];
        } else if vec.len() == 4 {
            parameter_mode_two = vec[0];
            parameter_mode_one = vec[1];
        } else if vec.len() == 3 {
            parameter_mode_one = vec[0];
        }

        Instruction {
            first_parameter: parameter_mode_one,
            second_parameter: parameter_mode_two,
            third_parameter: parameter_mode_three,
            operation_mode: (number % 100),
        }
    }

    // Pass in parameter (eg. 1, 2 or 3) as index
    //   Returns the stored data
    fn get_parameter_value(&mut self, index: u32, data: &Vec<i32>, instruction_pointer: u32) -> i32 {
        match index {
            1 => {
                if self.first_parameter == 0 {
                   return data[data[(instruction_pointer + 1) as usize] as usize];
                } else {
                    return data[(instruction_pointer + 1) as usize];
                }
            },
            2 => {
                if self.second_parameter == 0 {
                   return data[data[(instruction_pointer + 2) as usize] as usize];
                } else {
                    return data[(instruction_pointer + 2) as usize];
                }
            },
            3 => {
                if self.third_parameter == 0 {
                   return data[data[(instruction_pointer + 3) as usize] as usize];
                } else {
                    return data[(instruction_pointer + 3) as usize];
                }
            },
            _ => { panic!("Unknown parameter index!"); }
        }
    }
}

// Helper function, split a number into a vec
fn number_to_vec(number: u32) -> Vec<u32> {
    number
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
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
