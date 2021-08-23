#![deny(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]
#![forbid(unsafe_code)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate num_derive;

mod game;

use game::Game;

fn main() {
    let game = Game::default();

    game.run();
}
