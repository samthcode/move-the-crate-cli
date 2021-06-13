mod game;
mod utilities;

fn main() {
    let mut game = game::Game::new();
    game.play();
}
