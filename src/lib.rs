use std::fmt::{Display, Formatter};

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
        match self.position {
            None => false,
            Some(_) => true,
        }
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = Some(position);
    }
}

impl Display for CheckedLetter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.position() {
            Some(position) => match position {
                Position::Correct => write!(f, "\x1b[1m{}\x1b[0m", self.letter()),
                Position::Wrong => write!(f, "\x1b[3m{}\x1b[0m", self.letter()),
                Position::NotInWord => write!(f, "\x1b[2m{}\x1b[0m", self.letter()),
            },
            None => write!(f, "{}", self.letter()),
        }
    }
}

pub fn compare(input: [char; 5], target: [char; 5]) -> [CheckedLetter; 5] {
    let mut positions: [CheckedLetter; 5] = [
        CheckedLetter::new(input[0]),
        CheckedLetter::new(input[1]),
        CheckedLetter::new(input[2]),
        CheckedLetter::new(input[3]),
        CheckedLetter::new(input[4]),
    ];

    for (index, positioned_letter) in positions.iter_mut().enumerate() {
        if positioned_letter.letter() == target[index] {
            positioned_letter.set_position(Position::Correct);
        }
    }

    let mut leftover_letters: Vec<char> = target
        .into_iter()
        .zip(positions.iter())
        .filter(|(_, pos)| !pos.checked())
        .map(|(chr, _)| chr)
        .collect();

    loop {
        let unprocessed_letters: Vec<&mut CheckedLetter> = positions
            .iter_mut()
            .filter(|position| !position.checked())
            .collect();

        if unprocessed_letters.is_empty() {
            break;
        }

        for unprocessed_letter in unprocessed_letters {
            match leftover_letters
                .iter()
                .position(|chr| *chr == unprocessed_letter.letter())
            {
                Some(index) => {
                    unprocessed_letter.set_position(Position::Wrong);
                    leftover_letters.remove(index);
                    break;
                }
                None => {
                    unprocessed_letter.set_position(Position::NotInWord);
                    continue;
                }
            }
        }
    }

    positions
}

pub fn print_result(result: &[CheckedLetter], newline: bool) {
    for positioned_letter in result {
        print!("{}", positioned_letter);
    }

    if newline {
        println!();
    }
}
