use std::{fs::File, error::Error, io::{BufReader, BufRead}, collections::HashMap};

fn check_if_msg_is_unique(msg: &Vec<char>) -> bool {
    
    // inificiient since we create a new map each time
    let mut msg_chars: HashMap<&char, usize> = HashMap::new();

    for (idx, c) in msg.iter().enumerate() {
        match msg_chars.get(&c) {

            Some(_) => {

                return false;

            },

            None => {
                msg_chars.insert(c, idx);
            }
        }
    }

    return true;
}


fn main() -> Result<(), Box<dyn Error + 'static>> {
    
    let path = "input.txt";

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {

        let mut msg_vec: Vec<char> = Vec::new();

        'inner: for (i, c) in line?.chars().enumerate() {

            if i < 14 {

                msg_vec.push(c);

            } else {

                msg_vec.remove(0);

                msg_vec.push(c);

                dbg!(&msg_vec);
                let msg_unique: bool = check_if_msg_is_unique(&msg_vec);

                if msg_unique {
                    println!("{}", i + 1);
                    break 'inner;
                }
                // check if c is in msg
            }


        }

    }
    

    Ok(())
}
