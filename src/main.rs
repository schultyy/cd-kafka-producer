use fake::faker::lorem::en::*;
use fake::Fake;

fn produce_words() -> String {
    let words: Vec<String> = Words(3..5).fake();
    words.join(",")
}

fn main() {
    loop {
        let words = produce_words();
        println!("{}", words);
    }
}

#[cfg(test)]
mod tests {
    use crate::produce_words;

    #[test]
    fn test_produces_words() {
        let words = produce_words();
        assert!(words.len() > 0, "Expected concatenated word list, got empty string");
    }
}


