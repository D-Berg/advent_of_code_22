use std::{fs::File, io::{BufReader, BufRead}, error::Error};



fn main() -> Result<(), Box<dyn Error + 'static>> {


    let path = "input.txt";

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let win = 6;
    let draw = 3;
    let loss = 0;

    
    let rock = 1;
    let paper = 2;
    let scissor = 3;

    let mut total_score: i32 = 0;

    for line in reader.lines() {
        
        let line_string = line?;
        
        let line_split: Vec<&str> = line_string.split_whitespace().collect();

        let enemy_strat = line_split[0];

        let my_strat = line_split[1];


        match enemy_strat {

            "A" => { // rock

                match my_strat {
                    "X" => { 
                        total_score += &scissor + &loss;
                    },

                    "Y" => { // win paper 
                        total_score += &rock + &draw;
                    },

                    "Z" => { // lost scissors
                        total_score += &paper + &win;
                    },

                    _ => { panic!("No other strat exits for my strat"); }

                }

            },

            "B" => { // paper

                match my_strat {
                    "X" => { 
                        total_score += &rock + &loss;
                    },

                    "Y" => { 

                        total_score += &paper + &draw;
                    },

                    "Z" => { 
                        total_score += &scissor + &win;

                    },

                    _ => { }

                }
            },

            "C" => { // scissors

                match my_strat {
                    "X" => { 
                        total_score += &paper + &loss;
                    },

                    "Y" => { 

                        total_score += &scissor + &draw;
                    },

                    "Z" => { 
                        total_score += &rock + &win;

                    },

                    _ => { }

                }
            }, 

            _ => {

                panic!("file is badly formatted");
            }
        }

    }

    println!("Your total score: {}", total_score);

    Ok(())
}
