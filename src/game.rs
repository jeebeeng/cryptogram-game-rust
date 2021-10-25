static letters: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
               'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub struct Game {
    quote: &str,
    mapping: HashMap<char, (char, char)>,
}

impl Game {
    pub fn new(quote: &str);

    pub fn update(key: char, guess: char);

    pub fn check() -> bool;

    fn quote_letters() -> [char];

    fn scrambled_quote() -> &str;
}
