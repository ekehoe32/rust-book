use std::io;

fn main() {
    // set constants
    const F2C_CONV_FACTOR: f64 = 5.0 / 9.0;
    const ZERO_CELSIUS_IN_F: f64 = 32.0;

    let farenheit: f64 = loop {
        // get user input
        println!("Input a temperature in Farenheit:");
        let mut farenheit = String::new();
        match io::stdin().read_line(&mut farenheit) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read line!");
                continue;
            }
        }

        // parse as float
        match farenheit.trim().parse() {
            Ok(f) => break f,
            Err(_) => {
                println!("Not a valid temperature!");
                continue;
            }
        };
    };

    // convert to C
    let celsius = F2C_CONV_FACTOR * (farenheit - ZERO_CELSIUS_IN_F);
    println!("{farenheit:.2}°F is {celsius:.2}°C!");
}
