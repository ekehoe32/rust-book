use std::io;
use thousands::Separable;

fn main() {
    loop {
        // read in input
        println!("Input an integer:");
        let mut n = String::new();
        match io::stdin().read_line(&mut n) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read line!");
                continue;
            }
        };

        // check the integer and compute the fibonacci number
        let n: i64 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Not an integer value!");
                continue;
            }
        };

        // print the result and end the program
        println!(
            "The {}th fibonacci number is: {}",
            n.separate_with_commas(),
            fibonacci(n).separate_with_commas()
        );
        break;
    }
}

fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else {
        return n + fibonacci(n - 1);
    }
}
