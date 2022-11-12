mod lib;

use crate::lib::{compare, print_result, Position};
use std::io::{stdin, stdout, Write};

const MAX_TRIES: u8 = 6;
const TARGET_WORD: &str = "WEEPY";

fn main() {
    let mut tries_left: u8 = MAX_TRIES;

    while tries_left > 0 {
        let word = read_word();
        let result = compare(
            word,
            TARGET_WORD
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
        );
        print_result(&result, true);

        if result
            .iter()
            .all(|letter| letter.position() == &Some(Position::Correct))
        {
            println!("Congrats, you won!");
            return;
        }

        tries_left -= 1;
        println!("Tries left: {}", tries_left);
    }

    println!("You lost!");
}

fn read_word() -> [char; 5] {
    let mut trimmed: String;

    loop {
        print!("Enter a 5-letter word: ");
        stdout().flush().expect("Cannot flush STDOUT.");
        let mut word = String::new();

        match stdin().read_line(&mut word) {
            Ok(_) => (),
            Err(_) => {
                continue;
            }
        };

        trimmed = word
            .trim()
            .chars()
            .map(|chr| chr.to_ascii_uppercase())
            .collect();

        if !trimmed.chars().all(|chr| chr.is_alphabetic()) {
            continue;
        }

        if trimmed.len() == 5 {
            break;
        }
    }

    trimmed.chars().collect::<Vec<char>>().try_into().unwrap()
}
