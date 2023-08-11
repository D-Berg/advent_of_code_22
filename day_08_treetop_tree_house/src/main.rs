use std::{fs::File, error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error + 'static>>{

    // Read in ---------------------------------------------------------------

    let path = "input.txt";

    let file = File::open(path)?;

    let reader = BufReader::new(file);


    let mut tree_map: Vec<Vec<u32>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {

        tree_map.push(Vec::new());

        for (j, c) in line?.chars().enumerate() {

            let tree_height: u32 = c.to_digit(10).unwrap();
            tree_map[i].push(tree_height);
        }

    }

    
    // dbg!(&tree_map);

    // Compute ---------------------------------------------------------------

    let mut scenic_scores: Vec<i32> = Vec::new();
    
    // add edges since all of them are visible
    let rows: usize = tree_map.len();
    let cols: usize = tree_map[0].len();


    // traversing trees is O(MN)
    // checking if tree is visible is O(NM)?
    for i in 1..(rows - 1) {

        for j in 1..(cols - 1) {

            let current_tree = tree_map[i][j];

            let mut scenic_score = 1;
            let mut viewing_distance = 0;
            // check that all trees above are shorter than current tree

            'above: for k in (0..i).rev() {

                // if a tree above is equal or taller than
                // current tree is not visible from above.
                viewing_distance += 1;
                if tree_map[k][j] >= current_tree {
                    break 'above;
                }
                
            }
            
            if viewing_distance != 0 {
                scenic_score *= viewing_distance;
            }

            viewing_distance = 0;
            // check treees below 

            'below: for k in (i + 1)..rows {
                viewing_distance += 1;
                if tree_map[k][j] >= current_tree {
                    break 'below;
                }

            }

            if viewing_distance != 0 {
                scenic_score *= viewing_distance;
            }
            viewing_distance = 0;

            // check trees to the right

            'right: for l in (j + 1)..cols {
                viewing_distance += 1;
                if tree_map[i][l] >= current_tree {
                    break 'right;
                }
            }

            if viewing_distance != 0 {
                scenic_score *= viewing_distance;
            }
            viewing_distance = 0;
            // check trees from the left
            'left: for l in (0..j).rev() {
                
                viewing_distance += 1;
                if tree_map[i][l] >= current_tree {
                    break 'left;
                }

            }

            if viewing_distance != 0 {
                scenic_score *= viewing_distance;
            }
            scenic_scores.push(scenic_score);

        }
    }


    scenic_scores.sort();
    dbg!(scenic_scores);

    Ok(())
}

