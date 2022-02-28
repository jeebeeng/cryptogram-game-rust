mod game;
mod request;

use std::error::Error;
use game::Game;

const URL: &str = "https://api.quotable.io/random";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let quote: request::Quote = request::get_quote(URL.to_string()).await?;
    let game: game::Game = Game::new(&quote.content, &quote.author);

    println!("{:?}", quote);

    println!("{}", game.scrambled_quote());

    Ok(())
}
