use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Game<'a> {
    pub quote: &'a str,
    pub author: &'a str,
    mapping: HashMap<char, (char, char)>,
}

impl<'a> Game<'a> {
    pub fn new(quote: &'a str, author: &'a str) -> Game<'a> {
        // make the letters in the quote into a Vector of strings
        let quote_letters: Vec<char> = Game::quote_letters(&quote).into_iter().collect();

        // shuffles LETTERS and map each letter into a tuple with the
        // letter and the guess
        let mut letters = LETTERS.clone();
        letters.shuffle(&mut thread_rng());
        let letters: Vec<(char, char)> = letters
            .into_iter()
            .map(|letter| return (letter, ' '))
            .collect();

        // maps each letter in letters to a tuple containing
        // a letter in the quote and the guessed letter
        let mapping: HashMap<char, (char, char)> = quote_letters.into_iter().zip(letters).collect();

        Game {
            author,
            quote,
            mapping,
        }
    }

    pub fn update(&mut self, letter: &char, guess: &char) {
        self.mapping = self
            .mapping
            .iter()
            .map(|(k, (v, g))| {
                if v == letter {
                    (*k, (*v, *guess))
                } else {
                    (*k, (*v, *g))
                }
            })
            .collect::<HashMap<char, (char, char)>>();
    }

    pub fn check(&self) -> bool {
        for (_, (value, guess)) in &self.mapping {
            if value != guess {
                return false;
            }
        }
        true
    }

    pub fn scrambled_quote(&self) -> String {
        let mut scrambled_quote = String::new();

        for c in self.quote.to_uppercase().chars() {
            if LETTERS.contains(&c) {
                let value = match self.mapping.get(&c) {
                    Some((v, _)) => v.to_string(),
                    _ => String::from(""),
                };
                scrambled_quote.push_str(&value);
            } else {
                scrambled_quote.push_str(&c.to_string());
            }
        }

        scrambled_quote
    }

    fn quote_letters(quote: &str) -> Vec<char> {
        quote
            .to_uppercase()
            .chars()
            .unique()
            .filter(|c| LETTERS.contains(c))
            .collect()
    }
}
