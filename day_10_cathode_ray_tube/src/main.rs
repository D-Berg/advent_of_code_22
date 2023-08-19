use std::{fs::File ,error::Error, io::{BufReader, BufRead}, usize, fmt::Display};
use Instruction::*;


const CRT_PIXEL_WIDTH: usize = 40;
const CRT_PIXEL_HEIGHT: usize = 6;

#[derive(Debug)]
struct Crt {
    display: [[bool; CRT_PIXEL_WIDTH]; CRT_PIXEL_HEIGHT]
}

impl Crt {
    
    fn new() -> Self {

        Crt { display: [[false; CRT_PIXEL_WIDTH]; CRT_PIXEL_HEIGHT] }

    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        for i in 0..CRT_PIXEL_HEIGHT {
            for j in 0..CRT_PIXEL_WIDTH {

                if self.display[i][j] {

                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }

            }
            
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Cpu {
    x: i32,
    cycle: i32,
    stack: Vec<Instruction>,
    signal_strengths: Vec<i32>,
    signal_strength_update_values: Vec<i32>,
    crt: Crt,
}

impl Cpu {

    fn new() -> Self {
        let signal_strength_update_values: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

        Cpu { 
            x: 1,
            cycle: 0,
            stack: Vec::new(),
            signal_strengths: Vec::new(),
            signal_strength_update_values,
            crt: Crt::new()
        }
    }

    fn push_instruction(&mut self, instruction: Instruction) {

        self.stack.push(instruction);

    }

    fn execute_stack(&mut self) {

        self.stack.reverse();

        // while their exists instruction on the stack
        while !self.stack.is_empty() {

            self.cycle += 1;


            let instruction = self.stack.pop().unwrap();

            println!("Start of cycle {}: register has value {}", &self.cycle, &self.x);

            if self.signal_strength_update_values.contains(&self.cycle) {

                let signal_stength = self.cycle * self.x;

                self.signal_strengths.push(signal_stength);

            }

            // dbg!(&instruction);
            // dbg!(&self.cycle);
            let p_y = (self.cycle - 1) / 40;
            let p_x = (self.cycle - 1) % 40;

            
            dbg!(&p_y, &p_x);

            let sprite_pos = vec![self.x -1, self.x, self.x + 1];

            dbg!(&sprite_pos);

            // sprite is visible
            if sprite_pos.contains(&p_x) {
                self.crt.display[p_y as usize][p_x as usize] = true;
            } 

            match instruction {
                
                NOOP => {},
                ADDX(info) => {

                    if info.0 > 0 {
                        self.stack.push( ADDX( (0, info.1) ) );
                    } else {
                        self.x += info.1;
                    }
                }
            }

            // execute instruction

            println!("End of cycle {}: register has value {} \n", &self.cycle, &self.x);




        }

    }

}

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX((i32, i32)) // first value is number of instructions to complete, second is v.
}

fn main() -> Result<(), Box<dyn Error + 'static>> {

    let path = "input.txt";
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut cpu = Cpu::new();

    // add instructions to cpu stack
    for line in reader.lines() {

        let line_string = line?;

        let line_split: Vec<&str> = line_string.split_whitespace().collect();

        match line_split[0] {
            "noop" => {

                cpu.push_instruction(Instruction::NOOP);

            },

            "addx" => {

                let v: i32 = line_split[1].parse()?;
                cpu.push_instruction(Instruction::ADDX((1, v)));

            },

            _ => {
                panic!("Faulty instruction given from input");
            }
        }

    }

    cpu.execute_stack();

    dbg!(&cpu.crt.display);
    println!("{}", cpu.crt);

    let answer: i32 = cpu.signal_strengths.iter().sum();
    println!("{}", answer);
    Ok(())

}

