use rustywords::{get_random_word, guess};

fn main() {
    match get_random_word() {
        Ok(word) => guess(word),
        Err(msg) => eprintln!("{}", msg),
    }
}
