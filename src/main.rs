use mtgrust::card_data::test_serialize;

fn main() {
    for x in test_serialize() {
        println!("{}", x);
        println!();
    }
}
