use std::collections::{HashMap, HashSet};

use clap::Parser;
use word_frequency::tokenizer::{Token, Tokenizer};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    file_name: String,
}

fn main() {
    let file_name = Args::parse().file_name;
    match std::fs::read_to_string(&file_name) {
        Ok(contents) => handle_string(contents.to_lowercase()),
        Err(e) => print_error_and_exit(e, 2),
    }
}

fn handle_string(contents: String) {
    let stop_words = read_stop_words();
    let mut tokenizer = Tokenizer::new(&contents);
    let mut word_frequencies: HashMap<String, usize> = HashMap::<String, usize>::new();
    while let Some(token) = tokenizer.next_token() {
        if let Token::Word(word) = token
            && !stop_words.contains(&word)
        {
            word_frequencies
                .entry(word)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    let mut v: Vec<_> = word_frequencies.iter().collect();
    v.sort_by(|a, b| compare_counts(a, b));
    for i in 0..std::cmp::min(25, v.len()) {
        println!("{} - {}", v[i].0, v[i].1);
    }
}

fn compare_counts(a: &(&String, &usize), b: &(&String, &usize)) -> std::cmp::Ordering {
    if b.1 == a.1 {
        return a.0.cmp(b.0);
    } else {
        return b.1.cmp(a.1);
    }
}

fn read_stop_words() -> HashSet<String> {
    match std::fs::read_to_string("stop-words.txt") {
        Ok(contents) => contents.split(',').map(|s| s.trim().to_string()).collect(),
        Err(e) => {
            print_error_and_exit(e, 3);
        }
    }
}

fn print_error_and_exit(error: std::io::Error, exit_code: i32) -> ! {
    eprintln!("Error opening file: {}", error);
    std::process::exit(exit_code);
}
