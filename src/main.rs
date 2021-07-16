mod game;

use game::Game;

fn main() {
    let game = Game::default();

    game.run();
}
