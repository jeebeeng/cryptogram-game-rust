use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;

const LETTERS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
               'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub struct Game<'a> {
    quote: &'a str,
    mapping: HashMap<String, (char, char)>,
}

impl<'a> Game<'a> {
    pub fn new(quote: &'a str) -> Game<'a> {
        // make the letters in the quote into a Vector of strings
        let quote_letters: Vec<String> = Game::quote_letters(&quote).into_iter()
            .map(|letter| letter.to_string()).collect();

        // shuffles LETTERS and map each letter into a tuple with the
        // letter and the guess
        let mut letters = LETTERS.clone();
        letters.shuffle(&mut thread_rng());
        let letters: Vec<(char, char)> = letters.into_iter()        
            .map(|letter| return (letter, ' '))
            .collect();

        // maps each letter in letters to a tuple containing 
        // a letter in the quote and the guessed letter
        let mapping: HashMap<String, (char, char)> = 
            quote_letters
            .into_iter()
            .zip(letters)
            .collect();

        Game {
            quote,
            mapping,
        }
    }

    pub fn update(&mut self, letter: &char, guess: &char) {
        for (key, (value, _)) in &self.mapping.clone() {
            if value == letter {
                if let Some(_) = self.mapping.insert(key.to_string(), (*value, *guess)) {
                    return ();
                }
            }
        }
    }

    pub fn check(&self) -> bool {
        for (_, (value, guess)) in &self.mapping.clone() {
            if value != guess {
                return false;
            }
        }
        true
    }

    fn quote_letters(quote: &str) -> Vec<char> {
        quote.to_uppercase()
            .chars()
            .filter(|c| LETTERS.contains(c))
            .collect()
    }

    fn scrambled_quote(&self) -> String {
        String::from("")
    }
}
