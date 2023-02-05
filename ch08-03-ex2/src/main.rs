use std::io;

fn main() {
    loop {
        // get word from user
        let mut word = String::new();
        println!("Enter a word:");
        match io::stdin().read_line(&mut word) {
            Ok(_) => (),
            Err(_) => {
                println!("Error when reading the word, please try again!");
                continue;
            }
        }

        // check it is english
        let word = word.trim();
        match is_english(&word) {
            true => (),
            false => {
                println!(
                    "The word entered is not made up of the English alphabet, please try again!"
                );
                continue;
            }
        }

        // make it pig latin
        println!("Your word in pig-latin translates to {}", pig_latin(&word));
        break;
    }
}

fn is_english(word: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    return word.chars().all(|x| alphabet.contains(x));
}

fn pig_latin(word: &str) -> String {
    let vowels = "aeiouy";
    let first_letter = word.chars().collect::<Vec<char>>()[0];
    return match vowels.contains(first_letter) {
        true => format!("{word}-hay"),
        false => format!("{}-{}ay", &word[1..], first_letter),
    };
}
