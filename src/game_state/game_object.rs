use crate::card_data::CardData;

pub struct CardObject<'card> {
    card_data: &'card CardData,
}
