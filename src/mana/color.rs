use chumsky::prelude::*;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Color::White => "W",
            Color::Blue => "U",
            Color::Black => "B",
            Color::Red => "R",
            Color::Green => "G",
        })
    }
}

impl TryFrom<char> for Color {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'W' => Ok(Color::White),
            'U' => Ok(Color::Blue),
            'B' => Ok(Color::Black),
            'R' => Ok(Color::Red),
            'G' => Ok(Color::Green),
            bad => Err(format!("{} is not a valid color", bad)),
        }
    }
}

pub fn color_parser() -> impl Parser<char, Color, Error = Simple<char>> {
    any().try_map(|c, span| {
        Color::try_from(c)
            .map_err(|_| Simple::expected_input_found(span, "WUBRG".chars().map(Some), Some(c)))
    })
}
