// This program is free software. It comes without any warranty, to
// the extent permitted by applicable law. You can redistribute it
// and/or modify it under the terms of the Do What The Fuck You Want
// To Public License, Version 2, as published by Sam Hocevar. See
// http://www.wtfpl.net/ for more details.

mod commandline;
mod lottery;

use crate::lottery::{Game, generate};
use clap::Parser;

fn main() {
    let games = Game::load();
    let config = commandline::Config::parse();
    // Find the game by name
    for game in &games {
        if game.id == config.game {
            println!("Game: {}\n", game.name);
            for i in 0..config.tickets {
                match generate(&game) {
                    Some(number) => println!("{}: {}", i + 1, number),
                    None => {}
                }
            }
        }
    }
}
