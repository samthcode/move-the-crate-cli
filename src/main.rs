use game::Game;

mod game;
mod utilities;

fn main() {
    let mut game = Game::new();
    game.play();
}
