use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

const MAX_TRIES: u8 = 6;
const WORD_SIZE: usize = 5;
const WORDS_FILE: &str = "./words.txt";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const ITALIC: &str = "\x1b[3m";
const RESET: &str = "\x1b[0m";

pub type Word = [char; WORD_SIZE];
pub type CheckedWord = [CheckedLetter; WORD_SIZE];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Position {
    Correct,
    Wrong,
    NotInWord,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CheckedLetter {
    letter: char,
    position: Option<Position>,
}

impl CheckedLetter {
    pub fn new(letter: char) -> Self {
        Self {
            letter,
            position: None,
        }
    }

    pub fn letter(&self) -> char {
        self.letter
    }

    pub fn position(&self) -> &Option<Position> {
        &self.position
    }

    pub fn checked(&self) -> bool {
        self.position.is_some()
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = Some(position);
    }
}

impl Display for CheckedLetter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.position() {
            Some(position) => match position {
                Position::Correct => write!(f, "{}{}{}", BOLD, self.letter(), RESET),
                Position::Wrong => write!(f, "{}{}{}", ITALIC, self.letter(), RESET),
                Position::NotInWord => write!(f, "{}{}{}", DIM, self.letter(), RESET),
            },
            None => write!(f, "{}", self.letter()),
        }
    }
}

pub trait Solvable {
    fn solved(&self) -> bool;
}

impl Solvable for CheckedWord {
    fn solved(&self) -> bool {
        self.iter()
            .all(|word| word.position().unwrap_or(Position::Wrong) == Position::Correct)
    }
}

pub trait WordParser: Sized {
    fn from_str(s: &str) -> Result<Self, String> {
        Self::from_trimmed_str(s.trim())
    }

    fn from_trimmed_str(s: &str) -> Result<Self, String>;
    fn read() -> Self;
}

impl WordParser for Word {
    fn from_trimmed_str(s: &str) -> Result<Self, String> {
        if !s.chars().all(|chr| chr.is_alphabetic()) {
            Err(format!("Not a word: {}", s))
        } else if s.len() != WORD_SIZE {
            Err(format!("Word must be of size: {}", WORD_SIZE))
        } else {
            Ok(s.chars()
                .map(|chr| chr.to_ascii_uppercase())
                .collect::<Vec<char>>()
                .try_into()
                .unwrap())
        }
    }

    fn read() -> Self {
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

            match Self::from_str(word.as_str()) {
                Ok(word) => {
                    return word;
                }
                Err(msg) => {
                    eprintln!("{}", msg);
                }
            }
        }
    }
}

pub fn compare(guess: Word, word: Word) -> CheckedWord {
    let mut positions: CheckedWord = guess.map(CheckedLetter::new);

    for (checked_letter, chr) in positions.iter_mut().zip(word) {
        if checked_letter.letter() == chr {
            checked_letter.set_position(Position::Correct);
        }
    }

    let mut leftover_letters: Vec<char> = word
        .into_iter()
        .zip(positions.iter())
        .filter(|(_, pos)| !pos.checked())
        .map(|(chr, _)| chr)
        .collect();

    for unprocessed_letter in positions.iter_mut().filter(|position| !position.checked()) {
        match leftover_letters
            .iter()
            .position(|chr| *chr == unprocessed_letter.letter())
        {
            Some(index) => {
                unprocessed_letter.set_position(Position::Wrong);
                leftover_letters.remove(index);
            }
            None => {
                unprocessed_letter.set_position(Position::NotInWord);
            }
        }
    }

    positions
}

pub fn print_result(result: &[CheckedLetter], newline: bool) {
    for letter in result {
        print!("{}", letter);
    }

    if newline {
        println!();
    }
}

pub fn get_random_word() -> Result<Word, &'static str> {
    let words: Vec<String> = read_words(WORDS_FILE)?.into_iter().collect();

    match words.choose(&mut rand::thread_rng()) {
        Some(string) => Ok(string.chars().collect::<Vec<char>>().try_into().unwrap()),
        None => Err("No word found."),
    }
}

pub fn guess(word: Word) {
    let mut tries_left: u8 = MAX_TRIES;

    while tries_left > 0 {
        let guess = Word::read();
        let result = compare(guess, word);
        print_result(&result, true);

        if result.solved() {
            println!("Congrats, you won!");
            return;
        }

        tries_left -= 1;
        println!("Tries left: {}", tries_left);
    }

    println!("You lost!");
}

fn read_words(filename: &str) -> Result<HashSet<String>, &'static str> {
    match File::open(filename) {
        Ok(file) => Ok(BufReader::new(file)
            .lines()
            .filter(|line| line.is_ok())
            .map(|line| line.unwrap().to_ascii_uppercase())
            .filter(|line| line.chars().all(|chr| chr.is_alphabetic()) && line.len() == WORD_SIZE)
            .collect()),
        Err(_) => Err("Could not read words file."),
    }
}
