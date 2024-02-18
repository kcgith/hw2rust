use std::io::{self, Write};

struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    fn count_words(&self) -> Result<usize, &'static str> {
        if self.text.trim().is_empty() {
            return Err("Error: Text is empty");
        }
        
        Ok(self.text.split_whitespace().count())
    }
}

fn main() {
    println!("Enter some text:");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let word_counter = WordCounter::new(&input);

    match word_counter.count_words() {
        Ok(word_count) => println!("Word count: {}", word_count),
        Err(err) => println!("{}", err),
    }
}
