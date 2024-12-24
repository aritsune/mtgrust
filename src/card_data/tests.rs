use serde::Deserialize;

use super::CardData;

#[derive(Debug, Deserialize)]
struct MtgJsonData {
    cards: Vec<CardData>,
}
#[derive(Debug, Deserialize)]
struct MtgJsonSet {
    data: MtgJsonData,
}

#[test]
fn deserialize_alpha() {
    use rand::seq::SliceRandom;
    use std::fs;
    let file =
        serde_json::de::from_str::<MtgJsonSet>(&fs::read_to_string("./data/LEA.json").unwrap())
            .unwrap();
    println!(
        "{:?}",
        file.data.cards.choose_multiple(&mut rand::thread_rng(), 3)
    )
}

#[test]
fn de_neo() {
    use rand::seq::SliceRandom;
    use std::fs;
    let file =
        serde_json::de::from_str::<MtgJsonSet>(&fs::read_to_string("./data/NEO.json").unwrap())
            .unwrap();
    println!(
        "{:#?}",
        file.data.cards.choose_multiple(&mut rand::thread_rng(), 3)
    )
}

#[test]
fn reconfigure() {
    let card: CardData = serde_json::from_value(
        serde_json::json!({"object":"card","id":"5d33a5b7-797b-4079-8d62-edd124c0fb5a","oracle_id":"c739e180-2f14-41ed-8e7e-50b7df985f35","multiverse_ids":[548461],"mtgo_id":97246,"arena_id":79588,"tcgplayer_id":262809,"cardmarket_id":608244,"name":"Rabbit Battery","lang":"en","released_at":"2022-02-18","uri":"https://api.scryfall.com/cards/5d33a5b7-797b-4079-8d62-edd124c0fb5a","scryfall_uri":"https://scryfall.com/card/neo/157/rabbit-battery?utm_source=api","layout":"normal","highres_image":true,"image_status":"highres_scan","image_uris":{"small":"https://cards.scryfall.io/small/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784","normal":"https://cards.scryfall.io/normal/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784","large":"https://cards.scryfall.io/large/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784","png":"https://cards.scryfall.io/png/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.png?1654567784","art_crop":"https://cards.scryfall.io/art_crop/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784","border_crop":"https://cards.scryfall.io/border_crop/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784"},"mana_cost":"{R}","cmc":1.0,
            "supertypes": [], "types": ["Artifact","Creature"], "subtypes": ["Rabbit","Equipment"]
            ,"oracle_text":"Haste\nEquipped creature gets +1/+1 and has haste.\nReconfigure {R} ({R}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)","power":"1","toughness":"1","colors":["R"],"color_identity":["R"],"keywords":["Haste","Reconfigure"]}),
    ).unwrap();
    println!("{:#?}", card);
}
