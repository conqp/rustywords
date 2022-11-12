use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const ITALIC: &str = "\x1b[3m";
const RESET: &str = "\x1b[0m";
const WORDS_FILE: &str = "./words.txt";

#[derive(Debug, Eq, PartialEq)]
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

pub fn compare(input: [char; 5], target: [char; 5]) -> [CheckedLetter; 5] {
    let mut positions: [CheckedLetter; 5] = input.map(CheckedLetter::new);

    for (checked_letter, chr) in positions.iter_mut().zip(target) {
        if checked_letter.letter() == chr {
            checked_letter.set_position(Position::Correct);
        }
    }

    let mut leftover_letters: Vec<char> = target
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

pub fn get_random_word() -> Result<[char; 5], &'static str> {
    let words: Vec<String> = read_words(WORDS_FILE)?.into_iter().collect();

    match words.choose(&mut rand::thread_rng()) {
        Some(string) => Ok(string.chars().collect::<Vec<char>>().try_into().unwrap()),
        None => Err("No word found."),
    }
}

fn read_words(filename: &str) -> Result<HashSet<String>, &'static str> {
    match File::open(filename) {
        Ok(file) => Ok(BufReader::new(file)
            .lines()
            .filter(|line| line.is_ok())
            .map(|line| line.unwrap().to_ascii_uppercase())
            .filter(|line| line.chars().all(|chr| chr.is_alphabetic()) && line.len() == 5)
            .collect()),
        Err(_) => Err("Could not read words file."),
    }
}
