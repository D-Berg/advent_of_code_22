use std::{fs::File, error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error + 'static>>{

    // Read in ---------------------------------------------------------------

    let path = "input.txt";

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    const ROWS: usize = 99;
    const COLS: usize = 100;

    let mut tree_map: [[u32; COLS]; ROWS] = [[0; COLS]; ROWS];

    for (i, line) in reader.lines().enumerate() {


        for (j, c) in line?.chars().enumerate() {

            let tree_height: u32 = c.to_digit(10).unwrap();

            tree_map[i][j] = tree_height;

        }

    }

    dbg!(&tree_map);

    // Compute ---------------------------------------------------------------

    let mut n_visible_trees: i32 = 0;
    
    // add edges since all of them are visible
    n_visible_trees += 2 * ROWS as i32; // add top and bottow edges
    n_visible_trees += 2 * (COLS - 2) as i32; // add side edges


    // TODO: compute which interior trees are visible
    // check interior

    // traversing trees is O(MN)
    // checking if tree is visible is O(NM)?
    for i in 1..(ROWS - 1) {

        for j in 1..(COLS - 1) {

            // TODO: check if tree (i, j) is visible
        }
    }


    println!("Number of visible trees: {}", n_visible_trees);

    Ok(())
}

