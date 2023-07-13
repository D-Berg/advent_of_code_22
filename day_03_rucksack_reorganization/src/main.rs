use std::{fs::File, error::Error, io::{BufReader, BufRead}, collections::HashMap};

fn main() -> Result<(), Box<dyn Error + 'static>> {


    // creation of alphabet
    let mut alphabet: Vec<char> = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect();

    let mut alphabet_upper_case: Vec<char> = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect();

    alphabet.append(&mut alphabet_upper_case);

    // create a hashmap for quick lookup of a letters value
    let mut value_table: HashMap<char, i32> = HashMap::new();

    for (i, c) in alphabet.iter().enumerate() {
        value_table.insert(*c, i as i32 + 1);
    }

    // dbg!(&value_table);

    let mut total_priority: i32 = 0;

    let path = "input.txt";

    let file = File::open(path)?;
    
    let reader = BufReader::new(file);

    

    let mut first_rucksack: HashMap<char, usize> = HashMap::new(); 
    let mut matches_with_second:HashMap<char, usize> = HashMap::new(); 

    

    let mut r = 1;


    for line in reader.lines() {

        let line_string = line?;
        

        dbg!(&total_priority);
        match r {
            1 => { // on the first r push all chars to a hashmap

                for (i, c) in line_string.chars().enumerate() {
                    first_rucksack.insert(c, i);
                }
                
            },

            2 => { // check for matches in the hashmap and create a new one with the matches

                for (i, c) in line_string.chars().enumerate() {

                    match first_rucksack.get(&c) {
                        Some(_) => { // found a match in between first and second rucksack
                            
                            matches_with_second.insert(c, i);

                        },
                        
                        None => {

                        }
                    }

                }
            },

            3 => {

                
                for c in line_string.chars() {
                    
                    match matches_with_second.get(&c) {
                        Some(_) => {

                            let prio_val = value_table.get(&c).unwrap();
                            dbg!(&prio_val);
                            dbg!(&c);
                            total_priority += prio_val;

                            first_rucksack.clear();
                            matches_with_second.clear();
                            r = 1;
                            continue;

                        },

                        None => {

                        }
                    }

                }

                first_rucksack.clear();
                matches_with_second.clear();
                r = 1;
                continue;



            },

            _ => {

            }

        }
            

        r += 1;


    }

    println!("Total priority: {}", total_priority);


    Ok(())
}
