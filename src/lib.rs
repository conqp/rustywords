use rand::seq::SliceRandom;
use std::array::IntoIter;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::slice::{Iter, IterMut};
use std::str::FromStr;

const WORD_SIZE: usize = 5;
const MAX_TRIES: u8 = 6;
const WORDS_FILE: &str = "./words.txt";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const ITALIC: &str = "\x1b[3m";
const RESET: &str = "\x1b[0m";

type Letters = [char; WORD_SIZE];
type CheckedLetters = [CheckedLetter; WORD_SIZE];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Position {
    Correct,
    Wrong,
    NotInWord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Word {
    letters: Letters,
}

impl Word {
    pub fn new(letters: Letters) -> Self {
        Self { letters }
    }

    pub fn read() -> Self {
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

    pub fn iter(&self) -> Iter<char> {
        self.letters.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<char> {
        self.letters.iter_mut()
    }

    pub fn map<T>(&self, f: fn(char) -> T) -> [T; WORD_SIZE] {
        self.letters.map(f)
    }
}

impl IntoIterator for Word {
    type Item = char;
    type IntoIter = IntoIter<Self::Item, WORD_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.letters.into_iter()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckedWord {
    letters: CheckedLetters,
}

impl CheckedWord {
    pub fn new(letters: CheckedLetters) -> Self {
        Self { letters }
    }

    pub fn solved(&self) -> bool {
        self.letters
            .iter()
            .all(|word| word.position().unwrap_or(Position::Wrong) == Position::Correct)
    }

    pub fn iter(&self) -> Iter<CheckedLetter> {
        self.letters.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<CheckedLetter> {
        self.letters.iter_mut()
    }

    pub fn map<T>(&self, f: fn(CheckedLetter) -> T) -> [T; WORD_SIZE] {
        self.letters.map(f)
    }
}

impl From<Word> for CheckedWord {
    fn from(word: Word) -> Self {
        Self::new(word.map(CheckedLetter::new))
    }
}

impl Display for CheckedWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.map(|letter| letter.to_string()).join(""))
    }
}

impl FromStr for Word {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !s.chars().all(|chr| chr.is_alphabetic()) {
            Err(format!("Not a word: {}", s))
        } else if s.len() != WORD_SIZE {
            Err(format!("Word must be of size: {}", WORD_SIZE))
        } else {
            Ok(Word::new(
                s.chars()
                    .map(|chr| chr.to_ascii_uppercase())
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap(),
            ))
        }
    }
}

pub fn compare(guess: Word, word: Word) -> CheckedWord {
    let mut positions: CheckedWord = CheckedWord::new(guess.map(CheckedLetter::new));

    for (checked_letter, chr) in positions.iter_mut().zip(word.into_iter()) {
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

pub fn get_random_word() -> Result<Word, String> {
    let words: Vec<Word> = read_words(WORDS_FILE)?.into_iter().collect();

    match words.choose(&mut rand::thread_rng()) {
        Some(word) => Ok(*word),
        None => Err("No word found.".to_string()),
    }
}

pub fn guess(word: Word) {
    let mut tries_left: u8 = MAX_TRIES;

    while tries_left > 0 {
        let guess = Word::read();
        let result = compare(guess, word);
        println!("{}", result);

        if result.solved() {
            println!("Congrats, you won!");
            return;
        }

        tries_left -= 1;
        println!("Tries left: {}", tries_left);
    }

    println!("You lost!");
}

fn read_words(filename: &str) -> Result<HashSet<Word>, &'static str> {
    match File::open(filename) {
        Ok(file) => Ok(BufReader::new(file)
            .lines()
            .filter(|line| line.is_ok())
            .map(|line| Word::from_str(&line.unwrap()))
            .filter(|word| word.is_ok())
            .map(|word| word.unwrap())
            .collect()),
        Err(_) => Err("Could not read words file."),
    }
}
