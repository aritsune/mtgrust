use mtgrust::card::test_serialize;

fn main() {
    for x in test_serialize() {
        println!("{}", x);
        println!();
    }
}
