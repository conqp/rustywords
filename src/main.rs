use rustywords::{compare, get_random_word, print_result, Position, Word, WORD_SIZE};
use std::io::{stdin, stdout, Write};

const MAX_TRIES: u8 = 6;

fn main() {
    match get_random_word() {
        Ok(word) => guess(word),
        Err(msg) => eprintln!("{}", msg),
    }
}

fn guess(word: Word) {
    let mut tries_left: u8 = MAX_TRIES;

    while tries_left > 0 {
        let guess = read_word();
        let result = compare(guess, word);
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

fn read_word() -> Word {
    let mut trimmed: String;

    loop {
        print!("Enter a {}-letter word: ", WORD_SIZE);
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

        if trimmed.len() == WORD_SIZE {
            break;
        }
    }

    trimmed.chars().collect::<Vec<char>>().try_into().unwrap()
}
