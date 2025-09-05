// This program is free software. It comes without any warranty, to
// the extent permitted by applicable law. You can redistribute it
// and/or modify it under the terms of the Do What The Fuck You Want
// To Public License, Version 2, as published by Sam Hocevar. See
// http://www.wtfpl.net/ for more details.

use rt_format::{FormatArgument, ParsedFormat, Specifier};
use std::{collections::HashMap, fmt};

use crate::lottery::{Game, random_in_range};

/// Lottery games should implement this trait
#[derive(Debug, PartialEq)]
enum Variant {
    Int(u8),
}

impl FormatArgument for Variant {
    fn supports_format(&self, _spec: &Specifier) -> bool {
        true
    }

    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => fmt::Display::fmt(&val, f),
        }
    }

    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }

    fn fmt_octal(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => fmt::Octal::fmt(&val, f),
        }
    }

    fn fmt_lower_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => fmt::LowerHex::fmt(&val, f),
        }
    }

    fn fmt_upper_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => fmt::UpperHex::fmt(&val, f),
        }
    }

    fn fmt_binary(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => fmt::Binary::fmt(&val, f),
        }
    }

    fn fmt_lower_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => fmt::LowerExp::fmt(&val, f),
        }
    }

    fn fmt_upper_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => fmt::UpperExp::fmt(&val, f),
        }
    }

    fn to_usize(&self) -> Result<usize, ()> {
        use std::convert::TryInto;
        match self {
            Variant::Int(val) => (*val).try_into().map_err(|_| ()),
        }
    }
}

pub fn generate(game: &Game) -> Option<String> {
    let mut all_numbers = vec![];

    for pool in &game.pools {
        let mut pool_numbers: Vec<u8> = (pool.range[0]..=pool.range[1]).collect();
        let mut numbers = vec![];
        for _count in 0..pool.count {
            let index = random_in_range(0, pool_numbers.len() as u8 - 1) as usize;
            numbers.push(Variant::Int(pool_numbers.remove(index)));
        }
        numbers.sort_by(|a, b| a.to_usize().unwrap().cmp(&b.to_usize().unwrap()));
        all_numbers.extend(numbers);
    }

    let binding = HashMap::<String, Variant>::new();
    let result = ParsedFormat::parse(&game.format, &all_numbers, &binding);

    match result {
        Err(_) => None,
        Ok(result) => Some(result.to_string()),
    }
}
