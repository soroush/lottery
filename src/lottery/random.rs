// This program is free software. It comes without any warranty, to
// the extent permitted by applicable law. You can redistribute it
// and/or modify it under the terms of the Do What The Fuck You Want
// To Public License, Version 2, as published by Sam Hocevar. See
// http://www.wtfpl.net/ for more details.

use ring::rand::{SecureRandom, SystemRandom};

/// Generate a random number between `low` and `high` inclusive without modulo bias
pub fn random_in_range(low: u8, high: u8) -> u8 {
    let rng: SystemRandom = SystemRandom::new();

    let range = (high - low + 1) as u32;
    let mut buf = [0u8; 4];

    loop {
        rng.fill(&mut buf).unwrap();
        let n = u32::from_le_bytes(buf);

        // Only accept values that won't cause modulo bias
        let max_acceptable = u32::MAX - u32::MAX % range;
        if n < max_acceptable {
            return (low as u32 + n % range) as u8;
        }
    }
}
