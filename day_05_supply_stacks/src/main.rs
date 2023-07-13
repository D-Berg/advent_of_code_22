use std::{error::Error, fs::File, io::{BufReader, BufRead, Write}, collections::HashMap, fmt::Display};

#[derive(Debug)]
struct Supplies {
    n_stacks: u32,
    stacks: HashMap<u32, Vec<char>>
}


impl Supplies {

    fn new() -> Supplies {

        Supplies {
            n_stacks: 0,
            stacks: HashMap::new(),
        }
    }

    fn create_stack(&mut self) {
        
        self.n_stacks += 1;

        self.stacks.insert(self.n_stacks, Vec::new());

    }

    fn push_crate(&mut self, stack_index: &u32, crate_char: char) {
 
        self.stacks.get_mut(stack_index).unwrap().push(crate_char);

    }

    fn pop_crate(&mut self, stack_index: &u32) -> char {
        
        self.stacks.get_mut(stack_index).unwrap().pop().unwrap()

    }

    fn print_top(&mut self) {

        for stack_idx in 1..=self.n_stacks {
            let top = self.stacks.get(&stack_idx).unwrap().last().unwrap();

            print!("{}", top);
        }

        std::io::stdout().flush().expect("failed to flush");
    }
    
}

impl Display for Supplies {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {


        // find out max height of stacks 
        // max len of the vectors

        //print!;
        //[c] if crate exists
        //3x<space> if crate doesnt
        //
        //print <space> after each print.

        let mut heights: Vec<usize> = Vec::new();
        for (_stack_idx, stack) in self.stacks.clone() {

            heights.push(stack.len());

        }

        let max_height = *heights.iter().max().unwrap();
        
        for l in (0..max_height).rev() {

            for stack_idx in 1..=self.n_stacks {
                
                let stack = self.stacks.get(&stack_idx).unwrap();

                match stack.get(l) {

                    Some(crt) => {
                        
                        write!(f, "[{}]", crt)?;

                    },
                    
                    None => {

                        write!(f, "   ")?;

                    }
                }

                write!(f, " ")?;

            }

            write!(f, "\n")?;

        }

        write!(f, " ")?;
        
        for i in 1..=self.n_stacks {
            write!(f, "{}   ", i)?;
        }


        write!(f, "\n")

        
    }
    
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    
    let path = "input.txt";

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    // suppleas is a hashmap of of stacks (vec)
    // key is stack index 

    let mut supplies: Supplies = Supplies::new();

    let mut supplies_strings: Vec<String> = Vec::new();
    let mut read_stacks: bool = true;

    for line in reader.lines() {

        let line_string: String = line?;

        match read_stacks {

            // read the lines that contains the stacks
            true => {

                dbg!(&line_string);

                match &line_string[..] {
                    "" => {

                        // TODO: push read lines to supplies 
                        
                        supplies_strings.reverse();

                        let mut stack_positions: Vec<usize> = Vec::new();

                        // create empty stack in supplies map
                        for (string_index, stack_index) in supplies_strings[0].chars().enumerate() {

                            if stack_index.is_numeric() {
                                // dbg!(&stack_index);
                                
                                supplies.create_stack();
                                stack_positions.push(string_index);

                            }
                        }

                        // dbg!(&supplies);
                        // dbg!(&stack_positions);

                        supplies_strings.remove(0);


                        for sup_string in supplies_strings.iter() {

                            let mut stack_index: u32 = 1;

                            // dbg!(&sup_string);
                            
                            let char_vec: Vec<char> = sup_string.chars().collect();

                            for stack_pos in stack_positions.iter() {

                                let c = char_vec[*stack_pos];

                                // push c to vec in hasmap(stack_index)
                                if c.is_alphabetic() {

                                    // dbg!(&c);
                                    // dbg!(&stack_index);
                                    supplies.push_crate(&stack_index, c);

                                }


                                stack_index += 1;
                            }

                        }

                        println!("{}", &supplies);

                        read_stacks = false;
                    },

                    _ => {
                        supplies_strings.push(line_string);
                    }
                }
            },


            // read the lines that contains the moves.
            false => {

                //TODO: do moves

                let instuctions: Vec<u32> = line_string
                    .split_whitespace()
                    .filter(|c| c.parse::<u32>().is_ok())
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect();

                let n_moves = instuctions[0];
                let from_stack = instuctions[1];
                let dest_stack = instuctions[2];

                let mut crates_to_move: Vec<char> = Vec::new(); 
                
                for _ in 0..n_moves {

                    //crate

                    
                    let crt: char = supplies.pop_crate(&from_stack);

                    crates_to_move.push(crt);

                }

                crates_to_move.reverse();

                for crt in crates_to_move.iter() {
                    supplies.push_crate(&dest_stack, *crt);
                }

                print!("\r{}", &supplies);

                // dbg!(n_moves);
                // dbg!(from_stack);
                // dbg!(dest_stack);
            }
        }
    }

    println!("{}", &supplies);
    // dbg!(&read_stacks);

    supplies.print_top();

    Ok(())


}
