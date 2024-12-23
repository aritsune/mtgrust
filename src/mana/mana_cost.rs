use super::color::Color;
use super::mana_symbol::{mana_symbol_parser, ManaSymbol};
use chumsky::prelude::*;
use std::fmt::Display;
use std::str::FromStr;

use Color::*;
use ManaSymbol::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ManaCost(pub Vec<ManaSymbol>);

impl ManaCost {
    pub fn iter(&self) -> std::slice::Iter<'_, ManaSymbol> {
        self.0.iter()
    }
    pub fn get_colors(&self) -> Vec<Color> {
        self.iter()
            .flat_map(|symbol| match symbol {
                Hybrid(one, two) | PhyrexianHybrid(one, two) => Some(vec![*one, *two]),
                Colored(c) | HybridColorless(c) | HybridGeneric(c, _) | Phyrexian(c) => {
                    Some(vec![*c])
                }
                Generic(_) | Placeholder(_) | Colorless | Snow => None,
            })
            .flatten()
            .collect()
    }
}

impl Display for ManaCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for symbol in self.iter() {
            f.write_fmt(format_args!("{}", symbol))?
        }
        Ok(())
    }
}

impl TryFrom<Vec<ManaSymbol>> for ManaCost {
    type Error = String;
    fn try_from(value: Vec<ManaSymbol>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Mana costs with zero symbols are not allowed!".to_owned());
        }
        Ok(Self(value))
    }
}

impl FromStr for ManaCost {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        mana_symbol_parser()
            .repeated()
            .at_least(1)
            .map(|symbols| ManaCost::try_from(symbols).unwrap())
            .parse(s)
            .map_err(|_| "Error parsing mana cost".to_owned())
    }
}

#[test]
fn test_mana_costs() {
    // Ajani, Sleeper Agent
    assert_eq!(
        ManaCost::from_str("{1}{G}{G/W/P}{W}").unwrap(),
        ManaCost(vec![
            Generic(1),
            Colored(Green),
            PhyrexianHybrid(Green, White),
            Colored(White)
        ])
    );
    // Altered Ego
    assert_eq!(
        ManaCost::from_str("{X}{2}{G}{U}").unwrap(),
        ManaCost(vec![
            Placeholder('X'),
            Generic(2),
            Colored(Green),
            Colored(Blue),
        ])
    );
}
