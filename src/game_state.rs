//use crate::card::Card;
//
//pub enum ZoneType {
//    Library,
//    Hand,
//    Battlefield,
//    Graveyard,
//    Stack,
//    Exile,
//    Command,
//}
//
//trait GameObject {}
//
//struct Spell {}
//impl GameObject for Spell {}
//
//struct CardObject<'a> {
//    card: &'a Card,
//}
//impl GameObject for CardObject<'_> {}
//impl<'a> From<&'a Card> for CardObject<'a> {
//    fn from(value: &'a Card) -> Self {
//        CardObject { card: &value }
//    }
//}
//
//trait Permanent: GameObject {
//    fn tap();
//    fn is_tapped();
//}
//
//struct Creature {
//    tapped: bool,
//}
//
//pub trait Zone {
//    fn get_objects(&self) -> impl Iterator<Item = &impl GameObject>;
//    fn add_object(&mut self, object: impl GameObject);
//    fn remove_object(&mut self, object: &impl GameObject) -> impl GameObject;
//}
//
//pub trait OrderedZone: Zone {
//    fn next(&self) -> &impl GameObject;
//}
//
//trait PrivateZone: Zone {}
//trait PublicZone: Zone {}
//trait HiddenZone: Zone {}
//
//pub struct Hand<'a> {
//    cards: Vec<CardObject<'a>>,
//}
//
//impl<'a> Zone for Hand<'a> {
//    fn get_objects(&self) -> impl Iterator<Item = &impl GameObject> {
//        self.cards.iter()
//    }
//    fn add_object(&mut self, object: impl GameObject) {
//        self.cards.push(object);
//    }
//}
//// impl PrivateZone for Hand {}
//impl<'a> Hand<'a> {
//    pub fn new() -> Hand<'a> {
//        Hand { cards: Vec::new() }
//    }
//}
//
//pub struct Library<'a> {
//    cards: Vec<CardObject<'a>>,
//}
//impl Zone for Library<'_> {
//    fn get_objects(&self) -> impl Iterator<Item = &impl GameObject> {
//        self.cards.iter()
//    }
//}
//// impl OrderedZone for Library {}
//// impl HiddenZone for Library {}
//impl<'a> Library<'a> {
//    // TODO: create Deck struct to use here instead
//    pub fn new(deck: &'a [Card]) -> Library {
//        Library {
//            cards: deck.iter().map(|c| CardObject::from(c)).collect(),
//        }
//    }
//}
//
//pub struct Graveyard {}
//impl Zone for Graveyard {
//    fn get_objects(&self) -> impl Iterator<Item = &impl GameObject> {
//        todo!()
//    }
//}
//impl PublicZone for Graveyard {}
//
//pub struct Player<'a> {
//    hand: Hand<'a>,
//    library: Library<'a>,
//    graveyard: Graveyard,
//    life: usize,
//}
//
//impl Player<'_> {
//    pub fn new(deck: &[Card]) -> Player<'_> {
//        Player {
//            hand: Hand::new(),
//            library: Library::new(deck),
//            graveyard: Graveyard {},
//            life: 20,
//        }
//    }
//}
//
//pub struct Battlefield {}
//impl Zone for Battlefield {
//    fn get_objects(&self) -> impl Iterator<Item = &impl GameObject> {
//        todo!()
//    }
//}
//impl PublicZone for Battlefield {}
//pub struct Exile {}
//impl Zone for Exile {
//    fn get_objects(&self) -> impl Iterator<Item = &impl GameObject> {
//        todo!()
//    }
//}
//impl PublicZone for Exile {}
//pub struct Stack {}
//impl Zone for Stack {
//    fn get_objects(&self) -> impl Iterator<Item = &impl GameObject> {
//        todo!()
//    }
//}
//impl PublicZone for Stack {}
//
//pub struct GameState<'a, 'b>
//where
//    'b: 'a,
//{
//    players: Vec<Player<'b>>,
//    battlefield: Battlefield,
//    exile: Exile,
//    stack: Stack,
//    previous_state: Option<&'a GameState<'a, 'b>>,
//}
//
//impl<'a, 'b> GameState<'a, 'b> {
//    pub fn new() -> GameState<'a, 'b> {
//        GameState {
//            players: vec![Player::new(&[])],
//            battlefield: Battlefield {},
//            exile: Exile {},
//            stack: Stack {},
//            previous_state: None,
//        }
//    }
//    pub fn move_object(
//        &mut self,
//        from: &mut impl Zone,
//        to: &mut impl Zone,
//        object: &impl GameObject,
//    ) {
//    }
//}
