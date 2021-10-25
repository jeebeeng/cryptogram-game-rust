use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;

const LETTERS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
               'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub struct Game {
    quote: String,
    mapping: HashMap<String, (char, char)>,
}

impl Game {
    pub fn new(quote: String) -> Game {
        let quote_letters = Game::quote_letters(&quote);
        let mut letters: Vec<String> = LETTERS.clone().into_iter()
            .map(|letter| letter.to_string()).collect();
        letters.shuffle(&mut thread_rng());

        let quote_letters: Vec<(char, char)> = quote_letters.into_iter()
                .map(|letter| return (letter, ' '))
                .collect();

        // scrambles LETTERS and maps them to a tuple containing 
        // a letter in the quote and the guessed letter
        let mapping: HashMap<String, (char, char)> = 
            letters
            .into_iter()
            .zip(quote_letters)
            .collect();

        Game {
            quote,
            mapping,
        }
    }

    pub fn update(&self, key: char, guess: char) {

    }

    pub fn check(&self) -> bool {
        true
    }

    fn quote_letters(quote: &str) -> Vec<char> {
        quote.to_uppercase()
            .chars()
            .filter(|c| LETTERS.contains(c))
            .collect()
    }

    fn scrambled_quote(&self) -> String {
        self.quote
    }
}
