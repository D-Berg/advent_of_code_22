use std::{
  error::Error, 
  fs::File, 
  io::{BufRead, BufReader}, 
  fmt::Display, 
  collections::HashMap
};

// distance/weight is 1 for all vertexes
type Vertex = [usize; 2];


type Graph = HashMap<Vertex, Vec<Vertex>>;

#[derive(Debug)]
enum Square {
    Height(i32),
    Start(i32),
    End(i32)
}


#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<Square>>,
    graph: Graph,
    path: HashMap<Vertex, (char, u32)>,
    start_position: [usize; 2],
    end_position: [usize; 2],
}

impl HeightMap {
    
    fn new() -> Self {
        HeightMap { 
            map: Vec::new(),
            graph: Graph::new(),
            path: HashMap::new(),
            start_position: [0, 0],
            end_position: [0, 0],
        }
    }

    fn build_graph(&mut self) {

        let n_rows = self.map.len();
        let n_cols = self.map[0].len();

        for i in 0..n_rows {

            for j in 0..n_cols {

                // check that we are not out of map
                // otherwise check up, down, left and right
                //
                let current_square = match self.map[i][j] {
                    Square::Start(v) => v,
                    Square::Height(v) => v,
                    Square::End(v) => v
                };

                self.graph.insert([i, j], Vec::new());


                // check right square
                if (j + 1) < n_cols {

                    let right_square = match self.map[i][j + 1] {
                        Square::Start(v) => v,
                        Square::Height(v) => v,
                        Square::End(v) => v
                    };

                    let jump = right_square - current_square;
                    
                    if jump == 0 || jump == 1 {

                        self.graph.get_mut(&[i, j]).unwrap().push([i, j + 1]);

                    }
                }

                // check left square
                if (j as i32 - 1) >= 0 { // need i32 since usize cant be less than 0

                    let left_square = match self.map[i][j - 1] {
                        Square::Start(v) => v,
                        Square::Height(v) => v,
                        Square::End(v) => v
                    };

                    let jump = left_square - current_square;
                    
                    if jump == 0 || jump == 1 {

                        self.graph.get_mut(&[i, j]).unwrap().push([i, j - 1]);

                    }
                }

                // check down square
                if (i + 1) < n_rows {

                    let down_square = match self.map[i + 1][j] {
                        Square::Start(v) => v,
                        Square::Height(v) => v,
                        Square::End(v) => v
                    };

                    let jump = down_square - current_square;
                    
                    if jump == 0 || jump == 1 {

                        self.graph.get_mut(&[i, j]).unwrap().push([i + 1, j]);

                    }
                }

                // check up square
                if (i as i32 - 1) >= 0 {

                    let up_square = match self.map[i - 1][j] {
                        Square::Start(v) => v,
                        Square::Height(v) => v,
                        Square::End(v) => v
                    };

                    let jump = up_square - current_square;
                    
                    if jump == 0 || jump == 1 {

                        self.graph.get_mut(&[i, j]).unwrap().push([i - 1, j]);

                    }
                }
            }
        }

    }

    /// Need to be called after build_graph()
    /// uses Dijkstra algorithm
    /// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
    fn find_shortest_path(&mut self) -> f64 {

        let mut queue: HashMap<Vertex, f64> = HashMap::new();

        // distance from source to node u.
        let mut dist: HashMap<Vertex, f64> = HashMap::new();

        // which nodes the algo has visited
        // will contain all nodes/vertices at end of algo

        let mut prev: HashMap<Vertex, Vertex> = HashMap::new();

        for vertex in self.graph.keys() {

            if vertex == &self.start_position {
                dist.insert(vertex.clone(), 0.0);
                queue.insert(vertex.clone(), 0.0);
            } else {
                dist.insert(vertex.clone(), f64::INFINITY);
                queue.insert(vertex.clone(), f64::INFINITY);
            }

        }

        while !queue.is_empty() {
            let v = get_vertex_with_min_dist(&queue);
            queue.remove(&v).unwrap();

            for u in self.graph.get(&v).unwrap() {

                let dist_v = dist.get(&v).unwrap();
                let alt = dist_v.clone() + 1.0;
                

                let dist_u = dist.get(u).unwrap();

                if &alt < dist_u {

                    *dist.get_mut(u).unwrap() = alt.clone();
                    *queue.get_mut(u).unwrap() = alt;

                    prev.insert(u.clone(), v.clone());

                }
                
            }


        }


        println!("{:#?}", &dist);
        
        dbg!("NOT STUCK YET");

        let mut curr = self.end_position.clone();

        let mut i: u32 = 0;

        'infinite_loop: while curr != self.start_position {

            dbg!(&i);

            if i > 10 {
                dbg!("NO PATH FOUND");
                break 'infinite_loop;
            }

            match prev.get(&curr) {

                Some(prev_pos) => {
                    
                    let diff = [
                        curr[0] as i32 - prev_pos[0] as i32, 
                        curr[1] as i32 - prev_pos[1] as i32
                    ];

                    let c: char = match diff {

                        [-1, 0] => '^',
                        [1, 0] => 'v',
                        [0, -1] => '<',
                        [0, 1] => '>',
                        _ => {'.'}
                    };

                    // TODO: fix coloring
                    
                    let val: f64 = 1.0 / (1.0 + f64::exp(i as f64 * 1000.0));
                    let val_int = (val * 255.0) as u32;

                    match self.path.get(&curr) {
                        Some(_) => break 'infinite_loop,
                        None => self.path.insert(curr, (c, val_int))
                    };

                    i += 1;
                    curr = prev.get(&curr).unwrap().clone();


                },

                None => break

            }

        }

        let min_dist = dist.get(&self.end_position).unwrap().clone();


        return min_dist;
    }
}


/// Prints out HeightMap as a colored map. 
impl Display for HeightMap {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for (i, map_row) in self.map.iter().enumerate() {


            for (j, sqr) in map_row.iter().enumerate() {

                
                match self.path.get(&[i, j]) {

                    Some(way) => {

                        let _ = match sqr {

                            Square::Height(h) => {
                                let h = h * 9;
                                // ascii codes. 
                                write!(f, "\x1b[48;2;0;0;{}m{}", way.1, way.0)
                            },
                            Square::Start(_) => write!(f, "{}", "S"),
                            Square::End(_) => write!(f, "{}", "E")
                            
                        };

                    },
                         
                    None => {

                        let _ = match sqr {

                            Square::Height(h) => {
                                let c = char::from_u32( (h + 97) as u32).unwrap();
                                let h = h * 9;
                                // ascii codes. 
                                write!(f, "\x1b[48;2;{h};0;0m{}", c)
                            },
                            Square::Start(_) => write!(f, "{}", "S"),
                            Square::End(_) => write!(f, "{}", "E")
                            
                        };
                    }
                    
                }


            }
            write!(f, "\x1b[0m\n")?;
        } 

        Ok(())
        
    }
}


/// Takes a hashmap of distances and returns the vertex
/// with the shortest distance.
fn get_vertex_with_min_dist(distances: &HashMap<Vertex, f64>) -> Vertex {

    let mut min_value = f64::INFINITY;
    let mut min_index: Vertex = [1000, 1000];

    for dist_key in distances.keys() {

        let dist = distances.get(dist_key).unwrap();

        if dist <= &min_value {
            min_value = dist.clone();
            min_index = dist_key.clone();
        }
    }

    return min_index;
}

fn main() -> Result<(), Box<dyn Error + 'static>> {

    let path = "input.txt";

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut height_map = HeightMap::new();

    // ------------ Read in input ---------------------
    for (row_idx, line) in reader.lines().enumerate() {

        let mut map_row: Vec<Square> = Vec::new();

        for (col_idx, c) in line?.chars().enumerate() {
            
            match c {

                'S' => {
                    height_map.start_position = [row_idx, col_idx];
                    map_row.push(Square::Start(0))
                },
                'E' => {
                    height_map.end_position = [row_idx, col_idx];
                    map_row.push( Square::End(25) )
                },
                // -97 to start 'a' at 0
                _ => map_row.push( Square::Height(c.to_ascii_lowercase() as i32 - 97) )

            }

        }

        height_map.map.push(map_row);
    }

    
    // ------------------- Solve problem------------

    height_map.build_graph();


    println!("{}", &height_map);

    // dbg!(&height_map.graph);

    let answer = height_map.find_shortest_path();

    println!("{}", &height_map);

    dbg!(&answer);


    Ok(())
}
