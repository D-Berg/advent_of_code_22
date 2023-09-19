use std::{fs::File, io::{BufReader, BufRead}, error::Error, usize};


// type WorryLevel = u32;
// type MonkeyIndex = usize;
// type Denominator = u32;

#[derive(Debug)]
enum Term {
    Number(u32),
    Old,
    None
}

#[derive(Debug)]
enum Operation {
    Multiplication,
    Addition,
    None
}


#[derive(Debug)]
struct Forumula {
    left_term: Term,
    right_term: Term,
    op: Operation
}

#[derive(Debug)]
struct Item {
    worry_level: u32,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Forumula,
    test: u32,
    if_false_true: (usize, usize),
    inspections: u32
}

impl Monkey {

    fn new() -> Self {

        Monkey { 
            items: Vec::new(), 
            operation: Forumula { 
                left_term: Term::None,
                right_term: Term::None,
                op: Operation::None
            },
            test: 0, 
            if_false_true: (0, 0),
            inspections: 0
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

    let mut monkey_idx: usize = 0;

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

                    let worry_level: u32 = line_split[i]
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


                let left_term: Term = match line_split[3] {

                    "old" => Term::Old,
                     _ => {
                        let num = line_split[3]
                            .parse()
                            .expect("Failed to parse num for op");
                        Term::Number(num)
                    }

                };


                let op: Operation = match line_split[4] {
                    "+" => Operation::Addition,
                    "*" => Operation::Multiplication,
                    _ => panic!("Operation do not exist"),
                };

                let right_term: Term = match line_split[5] {
                    "old" => Term::Old,
                    _ => {
                        let num = line_split[5]
                            .parse()
                            .expect("Failed to parse num for op lhs");

                        Term::Number(num)
                    }
                };

                monkey_group.monkeys[monkey_idx].operation = Forumula {
                    left_term,
                    right_term,
                    op
                }


            },

            "Test:" => {

                let test_val: u32 = line_split
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                monkey_group.monkeys[monkey_idx].test = test_val;

            },

            "If" => {

                match line_split[1] {
                    "true:" => {

                        let true_val: usize = line_split
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap();

                        monkey_group.monkeys[monkey_idx].if_false_true.1 = true_val;

                    },

                    "false:" => {

                        let false_val: usize = line_split
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
    

    let num_rounds: usize = 20;

    for round in 0..num_rounds {

        println!("Round {}", round + 1);
        

        for monkey_idx in 0..monkey_group.monkeys.len() {


            println!("   Monkey {} starts inspecting...", &monkey_idx + 1);
            // inspect items 
            
            monkey_group.monkeys[monkey_idx].items.reverse();

            while !monkey_group.monkeys[monkey_idx].items.is_empty() {

                // 1. look at item 
                monkey_group.monkeys[monkey_idx].inspections += 1;
                // 2. do operation on item
                
                let old_worry_lvl = &monkey_group
                    .monkeys[monkey_idx]
                    .items
                    .pop()
                    .unwrap()
                    .worry_level;

                println!("      Inspecting item with worry level: {}", &old_worry_lvl);

                let left: u32 = match monkey_group.monkeys[monkey_idx].operation.left_term {
                    Term::Number(num) => num,
                    Term::Old => old_worry_lvl.clone(),
                    Term::None => panic!("wrong input")
                };


                let right: u32 = match monkey_group.monkeys[monkey_idx].operation.right_term {
                    Term::Number(num) => num,
                    Term::Old => old_worry_lvl.clone(),
                    Term::None => panic!("wrong input")
                };


                let mut new_worry_level: u32 = match &monkey_group.monkeys[monkey_idx].operation.op {

                    Operation::Addition => left + right,

                    Operation::Multiplication => left * right,

                    Operation::None => panic!("Operation None: Should not exist")
                    
                };
                    
                println!("      new worry_level: {}", &new_worry_level);

                new_worry_level = new_worry_level / 3;

                println!("      devides by 3: {}", &new_worry_level);

                // 3. do test: devide by Denominator

                let test = monkey_group.monkeys[monkey_idx].test;

                println!("      Test: {}", &test);

                if new_worry_level % test == 0 { 

                    let monkey_to_throw_to = monkey_group
                        .monkeys[monkey_idx]
                        .if_false_true.1;
                    
                    println!("      Test is true, throwing to monkey {}", &monkey_to_throw_to);

                    monkey_group
                        .monkeys[monkey_to_throw_to]
                        .items
                        .push(
                            Item { 
                                worry_level: new_worry_level 
                            }
                        );

                } else {
                    
                    let monkey_to_throw_to = monkey_group
                        .monkeys[monkey_idx]
                        .if_false_true.0;

                    println!("      Test is false, throwing to monkey {}", &monkey_to_throw_to);
                    monkey_group
                        .monkeys[monkey_to_throw_to]
                        .items
                        .push(
                            Item { 
                                worry_level: new_worry_level 
                            }
                        );

                }
                
                



            }

        }

    }

    // dbg!(&monkey_group.monkeys);

    let mut monkey_inspections: Vec<u32> = Vec::new();

    for monkey in monkey_group.monkeys {
        monkey_inspections.push(monkey.inspections);
    }

    dbg!(&monkey_inspections);

    monkey_inspections.sort();
    monkey_inspections.reverse();

    let monkey_business = monkey_inspections[0] * monkey_inspections[1];
    
    println!("Level of monkey business: {}", monkey_business);
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


