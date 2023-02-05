use std::{collections::HashMap, io};

fn main() {
    loop {
        // get list of integers from user
        let mut input = String::new();
        println!("Input a list of comma separated integers:");
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Input could not be read, please try again.");
                continue;
            }
        };

        // parse the input into vector of integers
        let mut input: Vec<i32> = input
            .trim()
            .split(",")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        println!("Your parsed list is: {input:?}");

        // calculate the median
        input.sort();
        let median = input[input.len() / 2 as usize];
        println!("The median is: {median}");

        // calculate mode
        let mode = vec_mode(&input);
        println!("The mode is: {mode}");
        break;
    }
}

// functions
fn vec_mode(v: &Vec<i32>) -> i32 {
    let mut input_counts = HashMap::new();
    for value in v {
        let count = input_counts.entry(value).or_insert(0);
        *count += 1;
    }
    return **input_counts.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().0;
}
