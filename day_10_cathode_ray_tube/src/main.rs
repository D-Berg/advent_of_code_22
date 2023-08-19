use std::{fs::File ,error::Error, io::{BufReader, BufRead}};
use Instruction::*;


#[derive(Debug)]
struct Cpu {
    x: i32,
    cycle: i32,
    signal_strengths: Vec<i32>,
    signal_strength_update_values: Vec<i32>
}

impl Cpu {

    fn new() -> Self {
        let signal_strength_update_values: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

        Cpu { 
            x: 1,
            cycle: 0,
            signal_strengths: Vec::new(),
            signal_strength_update_values
        }
    }


    fn execute_instruction(&mut self, instruct: Instruction) {


        // check if cycle is 20th, 40th, ...
        //

        match instruct {

            NOOP => {

                self.cycle += 1;

                if self.signal_strength_update_values.contains(&self.cycle) {

                    let signal_stength = self.cycle * self.x;

                    self.signal_strengths.push(signal_stength);

                }
            }, 

            ADDX(v) => {

                for _ in 0..2 {

                    self.cycle += 1;

                    if self.signal_strength_update_values.contains(&self.cycle) {

                        let signal_stength = self.cycle * self.x;

                        self.signal_strengths.push(signal_stength);

                    }

                }

                self.x += v;

            }


        }
        
    }
}

enum Instruction {
    NOOP,
    ADDX(i32)
}

fn main() -> Result<(), Box<dyn Error + 'static>> {

    let path = "input.txt";
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut cpu = Cpu::new();
    for line in reader.lines() {

        let line_string = line?;

        let line_split: Vec<&str> = line_string.split_whitespace().collect();

        match line_split[0] {
            "noop" => {

                cpu.execute_instruction(Instruction::NOOP);

            },

            "addx" => {

                let v: i32 = line_split[1].parse()?;
                cpu.execute_instruction(Instruction::ADDX(v));

            },

            _ => {
                panic!("Faulty instruction given from input");
            }
        }

    }

    let answer: i32 = cpu.signal_strengths.iter().sum();
    println!("{}", answer);
    Ok(())

}

