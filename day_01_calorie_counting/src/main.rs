use std::{fs::File, error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error + 'static>> {

    let path = "input.txt";


    let file = File::open(path)?;

    let reader = BufReader::new(file);
    
    let mut calories: i32 = 0;

    let mut calories_vec: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_string = line?;
        
        // dbg!(&line_string);

        if line_string == "".to_string() {

            calories_vec.push(calories);

            calories = 0;

        } else {
            let line_calories = line_string.parse::<i32>()?;
            // dbg!(&line_calories);
            calories += line_calories;

        }

    }

    calories_vec.sort();
    calories_vec.reverse();

    let total_top_three = calories_vec[0] + calories_vec[1] + calories_vec[2];

    println!("Total: {}", total_top_three);


    Ok(())


}


