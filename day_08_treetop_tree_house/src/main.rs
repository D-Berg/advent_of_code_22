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

    let mut n_visible_trees: i32 = 0;
    
    // add edges since all of them are visible
    let rows: usize = tree_map.len();
    let cols: usize = tree_map[0].len();

    n_visible_trees += 2 * rows as i32; // add top and bottow edges
    n_visible_trees += 2 * (cols - 2) as i32; // add side edges

    dbg!(&n_visible_trees);
    // TODO: compute which interior trees are visible
    // check interior

    // traversing trees is O(MN)
    // checking if tree is visible is O(NM)?
    for i in 1..(rows - 1) {

        for j in 1..(cols - 1) {


            let mut tree_is_visible_from_above: bool = true;
            let mut tree_is_visible_from_below: bool = true;

            let mut tree_is_visible_from_right: bool = true;
            let mut tree_is_visible_from_left: bool = true;

            let current_tree = tree_map[i][j];

            // check that all trees above are shorter than current tree

            dbg!(&current_tree);
            
            'above: for k in 0..i {

                // if a tree above is equal or taller than
                // current tree is not visible from above.

                dbg!(tree_map[k][j]);

                if tree_map[k][j] >= current_tree {
                    tree_is_visible_from_above = false;
                    break 'above;
                }
                
            }

            dbg!(&tree_is_visible_from_above);

            if tree_is_visible_from_above {
                n_visible_trees += 1;
                // skip other checks.
                continue;
            }

            
            // check treees below 

            'below: for k in (i + 1)..rows {
                if tree_map[k][j] >= current_tree {
                    tree_is_visible_from_below = false;
                    break 'below;
                }
            }

            if tree_is_visible_from_below {
                n_visible_trees += 1;
                continue;
            }

            // check trees to the right

            'right: for l in (j + 1)..cols {
                if tree_map[i][l] >= current_tree {
                    tree_is_visible_from_right = false;
                    break 'right;
                }
            }

            if tree_is_visible_from_right {
                n_visible_trees += 1;
                continue;
            }
            // check trees from the left

            'left: for l in 0..j {
                
                if tree_map[i][l] >= current_tree {
                    tree_is_visible_from_left = false;
                    break 'left;
                }
            }

            if tree_is_visible_from_left {
                n_visible_trees += 1;
                continue;
            }


        }
    }


    println!("Number of visible trees: {}", n_visible_trees);

    let mut test_vec: Vec<Vec<i32>> = Vec::new();
    test_vec.push(Vec::new());
    test_vec[0].push(1);
    dbg!(test_vec[0][0]);

    Ok(())
}

