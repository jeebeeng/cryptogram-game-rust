use std::collections::HashMap;

const letters: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
               'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub struct Game {
    quote: String,
    mapping: HashMap<char, (char, char)>,
}

impl Game {
    pub fn new(quote: String) -> Game {
        let quote_letters = Game::quote_letters(quote);
        let mut mapping: HashMap<char, (char, char)> = 
            letters.clone()
            .into_iter()
            .zip(quote_letters
                .into_iter()
                .map(|letter| (letter, ' '))
                .collect())
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

    fn quote_letters(quote: String) -> Vec<char> {
        quote.chars().filter(|c| letters.contains(c)).collect()
    }

    fn scrambled_quote(&self) -> String {
        self.quote
    }
}
