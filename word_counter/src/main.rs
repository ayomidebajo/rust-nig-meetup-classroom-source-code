use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }
    fn increment(&mut self, word: &str) {
        let key = word.to_string();

        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }
    fn display(&self, filter: u64) {
        // keep data in a vector for sorting by storing it the same structure as the hashmap
        let mut vec_count: Vec<(&String, &u64)> = self.0.iter().collect();
        // sort by value
        vec_count.sort_by(|a, b| a.1.cmp(b.1));
        // print the sorted vector
        for (key, value) in vec_count {
            // print only the words that have a count greater than the filter
            if value > &filter {
                println!("{}: {}", key, value);
            }
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        panic!("Please provide a filename!");
    }
    let filename = &arguments[1];

    let file = File::open(filename).expect("Could not open file");

    let reader = BufReader::new(file);

    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");

        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.increment(&word);
            }
        }
    }
    word_counter.display(0);
}


#[cfg(feature = "second")]

#[test]
fn test_word_counter () {
    let mut word_counter = WordCounter::new();
    let random_words = ["this", "is", "just", "a", "list", "of", "words", "a", "words", "please"];

    assert!(random_words.len() > 0);

    for word in random_words.iter() {
        word_counter.increment(word);
    }

    assert!(word_counter.0.len() == 8_usize);
   dbg!("{}", word_counter.0.len());
    
}


#[cfg(feature = "default")]

#[test]
fn first_test() {
    assert!(2 == 2);
    assert_ne!(false, true);
    assert_eq!(1 + 1, 2);
}





