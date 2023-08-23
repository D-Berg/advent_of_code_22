use std::{fs::File, io::{BufReader, BufRead}, error::Error, usize};


type WorryLevel = u32;
type MonkeyIndex = usize;
type Denominator = u32;
type Operation = Vec<Term>;

#[derive(Debug)]
enum Term {
    Multiplication, 
    Addition,
    Number(i32),
    Old,
}



#[derive(Debug)]
struct Item {
    worry_level: WorryLevel
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Vec<Term>,
    test: Denominator,
    if_false_true: (MonkeyIndex, MonkeyIndex),
}

impl Monkey {

    fn new() -> Self {

        Monkey { 
            items: Vec::new(), 
            operation: Vec::new(),
            test: 0, 
            if_false_true: (0, 0)
        }
        

    }
    
}

struct MonkeyGroup {
    monkeys: Vec<Monkey>
}

impl MonkeyGroup {

    fn new() -> Self {

        let mut monkeys: Vec<Monkey> = Vec::new();

        for i in 0..8 {

            monkeys.push(Monkey::new());
        }

        MonkeyGroup { monkeys }
    }
    
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    println!("Hello, world!");

    let path = "input.txt";

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut monkey_group: MonkeyGroup = MonkeyGroup::new();

    let mut monkey_idx: MonkeyIndex = 0;

    // we know they are 7 monkeys


    // Parsing input
    for line in reader.lines() {

        let line_string = line?;
        
        if line_string.is_empty() {
            
            continue;

        }

        //filter out commas from the string
        let line_string: String = line_string
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .filter(|&c| *c != ',')
            .collect();

        let line_split: Vec<&str> = line_string.split_whitespace().collect();

        match line_split[0] {

            "Monkey" => {

            },

            "Starting" => {

                for i in 2..line_split.len() {

                    let worry_level: WorryLevel = line_split[i]
                        .parse()
                        .unwrap();

                    monkey_group
                        .monkeys[monkey_idx]
                        .items
                        .push(
                            Item { worry_level }
                        );

                }

            },

            "Operation:" => {

                for i in 3..line_split.len() {

                    match line_split[i] {

                        "old" => {

                            monkey_group
                                .monkeys[monkey_idx]
                                .operation
                                .push(Term::Old);

                        },

                        "+" => {

                            monkey_group
                                .monkeys[monkey_idx]
                                .operation
                                .push(Term::Addition);
                    

                        },

                        "*" => {

                            monkey_group
                                .monkeys[monkey_idx]
                                .operation
                                .push(Term::Multiplication);

                        },

                        _ => {

                            let num: i32 = line_split[i].parse().unwrap();

                            monkey_group
                                .monkeys[monkey_idx]
                                .operation
                                .push(Term::Number(num));

                        }


                    }

                }

            },

            "Test:" => {

                let test_val: Denominator = line_split
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                monkey_group.monkeys[monkey_idx].test = test_val;

            },

            "If" => {

                match line_split[1] {
                    "true:" => {

                        let true_val: MonkeyIndex = line_split
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap();

                        monkey_group.monkeys[monkey_idx].if_false_true.1 = true_val;

                    },

                    "false:" => {

                        let false_val: MonkeyIndex = line_split
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap();

                        monkey_group.monkeys[monkey_idx].if_false_true.0 = false_val;

                        monkey_idx += 1;

                    }, 

                    _ => {

                    }
                }
            },

            _ => {

            }

            
        }

        dbg!(&line_split);


    }

    dbg!(&monkey_group.monkeys);

    // COMPUTING 

    
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{WorryLevel, Operation, Term::*};


    #[test]
    fn test_operation() {
    
        let item: WorryLevel = 10;
        
        let mut test_op: Operation = Operation::new();

        test_op.push(Old);
        test_op.push(Addition);
        test_op.push(Number(2));

        
    }
}


