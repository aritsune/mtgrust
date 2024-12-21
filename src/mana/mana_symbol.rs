use super::color::{color_parser, Color};
use chumsky::prelude::*;
use std::{
    fmt::{Display, Write},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub enum ManaSymbol {
    Placeholder(char), // X, Y
    Generic(usize),
    Colored(Color),
    Colorless,
    Hybrid(Color, Color),
    HybridColorless(Color),
    HybridGeneric(Color, usize),
    Phyrexian(Color),
    PhyrexianHybrid(Color, Color),
    Snow,
}

use ManaSymbol::*;

impl Display for ManaSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('{')?;
        match self {
            Generic(num) => f.write_str(&num.to_string())?,
            Colorless => f.write_char('C')?,
            Snow => f.write_char('S')?,
            Colored(c) => f.write_str(&c.to_string())?,
            Hybrid(c_one, c_two) => f.write_fmt(format_args!("{}/{}", c_one, c_two))?,
            HybridColorless(c) => f.write_fmt(format_args!("{}/C", c))?,
            HybridGeneric(c, num) => f.write_fmt(format_args!("{}/{}", c, num))?,
            Phyrexian(c) => f.write_fmt(format_args!("{}/P", c))?,
            PhyrexianHybrid(c_one, c_two) => f.write_fmt(format_args!("{}/{}/P", c_one, c_two))?,
            &Placeholder(ch) => f.write_char(ch)?,
        };
        f.write_char('}')
    }
}

pub fn mana_symbol_parser() -> impl Parser<char, ManaSymbol, Error = Simple<char>> {
    let dec = text::int::<_, Simple<char>>(10);
    choice((
        dec.try_map(|digits, span| {
            let num: usize = str::parse(&digits).map_err(|_| {
                Simple::expected_input_found(span, "1234567890".chars().map(Some), Some('A'))
            })?;
            Ok(ManaSymbol::Generic(num))
        }),
        just("C").to(ManaSymbol::Colorless),
        just("S").to(ManaSymbol::Snow),
        color_parser()
            .then(just("/").ignore_then(color_parser()))
            .then(just("/").ignore_then(just("P")))
            .map(|((left, right), _)| ManaSymbol::PhyrexianHybrid(left, right)),
        color_parser()
            .then(just("/").ignore_then(just("P")))
            .map(|(color, _)| ManaSymbol::Phyrexian(color)),
        color_parser()
            .then(just("/").ignore_then(dec))
            .try_map(|(left, digits), span| {
                let num: usize = str::parse(&digits).map_err(|_| {
                    Simple::expected_input_found(span, "1234567890".chars().map(Some), Some('A'))
                })?;
                Ok(ManaSymbol::HybridGeneric(left, num))
            }),
        color_parser()
            .then(just("/").ignore_then(just("C")))
            .map(|(left, _)| ManaSymbol::HybridColorless(left)),
        color_parser()
            .then(just("/").ignore_then(color_parser()))
            .map(|(left, right)| ManaSymbol::Hybrid(left, right)),
        color_parser().map(ManaSymbol::Colored),
        any().map(ManaSymbol::Placeholder),
    ))
    .delimited_by(just("{"), just("}"))
}

impl FromStr for ManaSymbol {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        mana_symbol_parser().parse(s)
    }
}
