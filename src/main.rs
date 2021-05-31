mod game;

fn main() {
    let mut game = match game::Game::new() {
        Ok(game) => game,
        Err(err) => {
            println!("{}", err);
            return
        }
    };
    match game.play() {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
            return
        }
    };
}
