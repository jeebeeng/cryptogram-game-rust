mod game;

fn main() {
    let game = game::Game::new("Hello world!", "John Bang");
    println!("{}", game.scrambled_quote());
    println!("{}", game.quote);
}
