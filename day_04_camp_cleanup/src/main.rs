use std::{fs::File, io::{ BufReader, BufRead}, error::Error};

fn main() -> Result<(), Box<dyn Error + 'static>> {


    let path = "input.txt";

    let file = File::open(path)?;


    let reader = BufReader::new(file);

    let mut total_overlaps: i32 = 0;

    for line in reader.lines() {

        let line_string = line?;

        
        let pairs: Vec<&str>= line_string
            .split(|c| c == '-' || c == ',')
            .collect();

        let start1: i32 = pairs[0].parse()?;
        let end1: i32 = pairs[1].parse()?;
        let start2:i32 = pairs[2].parse()?;
        let end2: i32 = pairs[3].parse()?;
        

        // check if pair one contains pair2
    
        
        if start2 >= start1 && end2 <= end1 {
            total_overlaps += 1;
            continue;
        } 

        // check if pair2 contains pair 1
        if start1 >= start2 && end1 <= end2 {
            total_overlaps += 1;
            continue;
        }

        if start1 <= start2 && start2 <= end1 && end1 <= end2 {
            total_overlaps += 1;
            continue;
        }

        if start2 <= start1 && start1 <= end2 && end2 <= end1 {
            total_overlaps += 1;
            continue;
        }

        // dbg!(&line_string);

    }


    println!("Number of assighment pairs with overlaps: {}", total_overlaps);

    Ok(())
}
