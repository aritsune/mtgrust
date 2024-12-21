use super::ManaCost;
use super::ManaSymbol::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManaValue(usize);

impl From<&ManaCost> for ManaValue {
    fn from(cost: &ManaCost) -> Self {
        // this is a 'default' method used when all you have is a mana cost
        // more specific situations will have their own overrides
        let value = cost
            .iter()
            .map(|symbol| match symbol {
                &Generic(i) => i,
                // The following symbols only ever add 1 to mana value
                Colored(..) | Colorless | Hybrid(..) | HybridColorless(..) | Phyrexian(..)
                | PhyrexianHybrid(..) | Snow => 1,
                // X is 0 when determining mana value of a card not on the stack @CR 202.3e
                Placeholder(_) => 0,
                // if we ever have a color/0 symbol (this would be silly, yes)
                // then the higher of the two possible costs (1) should be used @CR 202.3f
                &HybridGeneric(_, i) => {
                    if i >= 1 {
                        i
                    } else {
                        1
                    }
                }
            })
            .sum();
        Self(value)
    }
}
