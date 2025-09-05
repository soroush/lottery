// This program is free software. It comes without any warranty, to
// the extent permitted by applicable law. You can redistribute it
// and/or modify it under the terms of the Do What The Fuck You Want
// To Public License, Version 2, as published by Sam Hocevar. See
// http://www.wtfpl.net/ for more details.

use clap;

#[derive(clap::Parser, Debug, Clone)]
pub struct Config {
    #[clap(long, name = "game", help = "Name of the game")]
    pub game: String,

    #[clap(
        long,
        name = "tickets",
        default_value_t = 1,
        help = "Number of tickets to generate (default is 1)"
    )]
    pub tickets: usize,
}
