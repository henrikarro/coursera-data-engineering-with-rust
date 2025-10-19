//!
//! Contains code to count the number of occurrences of words in a text.
//!
//! You can also provide an optional file with "stop words", i.e., small words that you want to ignore when counting.
//!
//! # Examples
//!
//! ```
//! use word_frequency::frequency::{Count, FrequencyCounter};
//!
//! let mut frequency_counter = FrequencyCounter::from_file("poem.txt").unwrap();
//! frequency_counter.read_stop_words("stop-words.txt").unwrap();
//! let word_frequencies = frequency_counter.count_frequencies();
//! let sorted_frequencies = FrequencyCounter::sort_frequencies(&word_frequencies);
//! assert_eq!(sorted_frequencies.len(), 15);
//! assert_eq!(sorted_frequencies[0], Count::new("nobody", 2));
//! assert_eq!(sorted_frequencies[14], Count::new("somebody", 1));
//! ```

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub struct FrequencyCounter {
    tokenizer: Tokenizer,
    stop_words: HashSet<String>,
}

impl FrequencyCounter {
    /// Creates a new `FrequencyCounter` that works on the given string, with no stop words.
    pub fn new(input: &str) -> Self {
        Self {
            tokenizer: Tokenizer::new(input),
            stop_words: HashSet::new(),
        }
    }

    /// Craeates a new `FrequencyCounter` by reading the given file and converting the contents into a string.
    pub fn from_file(file_name: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(file_name)?.to_lowercase();
        Ok(Self::new(&contents))
    }

    /// Reads a comma-separated file with "stop words", i.e., small words that you want to ignore when counting.
    pub fn read_stop_words(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        self.stop_words = std::fs::read_to_string(file_name)?
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        return Ok(());
    }

    /// Counts the number of offurrences of each word in the input string, ignoring the stop words.
    ///
    /// Returns a mappring from words to the number of occurrences of that word.
    pub fn count_frequencies(&mut self) -> HashMap<String, usize> {
        let mut word_frequencies: HashMap<String, usize> = HashMap::<String, usize>::new();
        while let Some(token) = self.tokenizer.next_token() {
            if let Token::Word(word) = token
                && !self.stop_words.contains(&word)
            {
                word_frequencies.entry(word).and_modify(|count| *count += 1).or_insert(1);
            }
        }
        word_frequencies
    }

    /// Given a mapping from words to frequencies as produced by [`FrequencyCounter::count_frequencies()`],
    /// returns a vector of [`Count`] objects sorted in descending order by count.
    pub fn sort_frequencies<'a>(word_frequencies: &'a HashMap<String, usize>) -> Vec<Count<'a>> {
        let mut v: Vec<_> = word_frequencies.iter().map(|wc| Count::new(wc.0, *wc.1)).collect();
        v.sort();
        v
    }
}

/// Contains a word and a count of the number of occurrences of that word.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Count<'a> {
    pub word: &'a str,
    pub count: usize,
}

impl<'a> Count<'a> {
    pub fn new(word: &'a str, count: usize) -> Self {
        Self { word, count }
    }
}

impl<'a> PartialOrd for Count<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Count<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.count == other.count {
            self.word.cmp(other.word)
        } else {
            other.count.cmp(&self.count)
        }
    }
}
