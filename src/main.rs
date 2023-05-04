use fake::faker::lorem::en::*;
use fake::Fake;

fn main() {
    loop {
        let words: Vec<String> = Words(3..5).fake();
        println!("words {:?}", words);
    }
}

