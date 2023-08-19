use core::fmt;
use std::{error::Error, fs::File, io::{BufReader, BufRead, Write}, ops::{Add, Sub, Mul}, collections::HashMap, fmt::Display, vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {

        Point { x, y }
       
    }

    fn norm(&self) -> f64 {

        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()

    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Point>,
    tail_history: HashMap<Point, usize>,
}

impl Rope {
    fn new() -> Rope {
        let knots = vec![Point::new(0, 0); 10];
        let mut tail_history: HashMap<Point, usize>= HashMap::new();
        
        tail_history.insert(Point::new(0, 0), 1);

        Rope { 
            knots,
            tail_history 

        }
    }

    fn move_rope(&mut self, steps: usize, direction: Direction) {

        for _ in 0..steps {

            let head_move_direction = match direction {
                Direction::Up => Point::new(0, 1),
                Direction::Down => Point::new(0, -1),
                Direction::Right => Point::new(1, 0),
                Direction::Left => Point::new(-1, 0)
            };

            self.knots[0] = self.knots[0] + head_move_direction;

            // if the tail is two steps below, above, right or move_left
            // move tail in corresponding way.

            for knot_idx in 1..self.knots.len() {
                let distance = self.knots[knot_idx - 1] - self.knots[knot_idx];
                let threshhold: f64 = 2.0;

                let distance_norm = distance.norm();
                // > sqrt(2)
                // is not adjacent
                if distance_norm > threshhold.sqrt() {
                    // move tail

                    // check if tail and head is in the same column 

                    self.knots[knot_idx] = self.knots[knot_idx] + distance * 0.5;

                    if knot_idx == (self.knots.len() - 1) {

                        match self.tail_history.get(&self.knots[knot_idx]) {

                            Some(_) => {

                                *self.tail_history.get_mut(&self.knots[knot_idx]).unwrap() += 1;
                            },

                            None => {

                                self.tail_history.insert(self.knots[knot_idx].clone(), 1);
                            }
                        }
                    }


                } 
            }




            // dbg!(&self.head_position);
            // dbg!(&self.tail_position);
                
        }

    }

}


impl Display for Rope {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let width: usize = 30;
        let height: usize = 30;

        let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];

        let knot_symbols: Vec<char> = vec!['H', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        for (knot_idx, knot) in self.knots.iter().enumerate() {

            grid[knot.y as usize + 14][knot.x as usize + 14] = knot_symbols[knot_idx];

        }

        for i in (0..height).rev() {

            for j in 0..width {

                write!(f, "{}", grid[i][j]).expect("errorrrr");
            }

            write!(f, "\n").unwrap();

        }

        Ok(())

    }

}

impl Add for Point {
    type Output = Self;

    fn add (self, other: Self) -> Self {

        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }

    }
}


impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self; 

    fn mul(self, rhs: f64) -> Self {

        let x: i32;
        let y: i32;

        if self.x == -1 { 
            x = (self.x as f64 * rhs).floor() as i32;
        } else {
            x = (self.x as f64 * rhs).ceil() as i32;
        }

        if self.y == -1 {
            y = (self.y as f64 * rhs).floor() as i32;
        } else {
            y = (self.y as f64 * rhs).ceil() as i32;
        }
        
        Point {
            x,
            y
        }
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>>{

    let mut rope: Rope = Rope::new();
    
    let path = "input.txt";
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {

        let line_string = line?;
        // dbg!(&line_string);
        let line_split: Vec<&str> = line_string.split_whitespace().collect();

        let move_type = line_split[0];

        let n_steps: usize = line_split[1].parse().unwrap();
        
        match move_type {

            "U" => {

                rope.move_rope(n_steps, Direction::Up);

            },

            "D" => {

                rope.move_rope(n_steps, Direction::Down);

            },

            "R" => {

                rope.move_rope(n_steps, Direction::Right);

            },

            "L" => {

                rope.move_rope(n_steps, Direction::Left);

            },

            _ => {
                panic!("move type should not exist");
            }
        }

        // println!("{}", &rope);
    }


    dbg!(&rope.tail_history.len());

    // dbg!(&rope.knots);
    // dbg!(&rope.tail_history);

    Ok(())
}


#[cfg(test)]
mod test {

    use crate::Point;

    #[test]
    fn test_point_distance() {

        let mut test_head = Point::new(0, -2);
        let mut test_tail = Point::new(0, 0);

        let distance = test_head - test_tail;

        // move tail
        
        // dbg!(&distance);

        let half_distance = distance * 0.5;
        // dbg!(&half_distance);
        test_tail = test_tail + distance * 0.5;

        // dbg!(&test_tail);
    }

    #[test]
    fn test_point_halfing() {
        let test_point1 = Point::new(-2, 0);
        let test_point2 = Point::new(0, -2);
        let test_point3 = Point::new(2, 1);
        let test_point4 = Point::new(-1, 2);
        let test_point5 = Point::new(2, -1);

        let half_point1 = test_point1 * 0.5;
        let half_point2 = test_point2 * 0.5;
        let half_point3 = test_point3 * 0.5;
        let half_point4 = test_point4 * 0.5;
        let half_point5 = test_point5 * 0.5;

        dbg!(&half_point1);
        dbg!(&half_point2);
        dbg!(&half_point3);
        dbg!(&half_point4);
        dbg!(&half_point5);

        let test: f64 = -0.5;

        dbg!(test.floor() as i32);


        assert_eq!(half_point1, Point::new(-1, 0));
        assert_eq!(half_point2, Point::new(0, -1));
        assert_eq!(half_point3, Point::new(1, 1));
        assert_eq!(half_point4, Point::new(-1, 1));
        assert_eq!(half_point5, Point::new(1, -1));
    }




}
