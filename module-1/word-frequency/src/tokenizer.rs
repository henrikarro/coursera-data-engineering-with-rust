#![allow(dead_code)]

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.to_string().chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.peek_char()? {
            c if c.is_whitespace() => self.read_whitespace(),
            c if c.is_alphabetic() => self.read_word(),
            c if c.is_numeric() => self.read_number(),
            c => self.read_punctuation(c),
        }
    }

    fn read_whitespace(&mut self) -> Option<Token> {
        self.position += 1;
        Some(Token::Whitespace)
    }

    fn read_word(&mut self) -> Option<Token> {
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch.is_alphabetic() {
                self.position += 1;
            } else if ch == '\'' && self.should_continue_reading_apostrophed_word() {
                continue;
            } else {
                break;
            }
        }
        Some(Token::Word(self.input[start..self.position].iter().collect()))
    }

    #[cfg(feature = "count-apostrophed-words-as-one")]
    fn should_continue_reading_apostrophed_word(&mut self) -> bool {
        if let Some(next_ch) = self.peek_next_char() {
            if next_ch.is_alphabetic() {
                self.position += 1; // consume apostrophe
                self.position += 1; // consume next alphabetic character
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    #[cfg(not(feature = "count-apostrophed-words-as-one"))]
    fn should_continue_reading_apostrophed_word(&mut self) -> bool {
        false
    }

    fn read_number(&mut self) -> Option<Token> {
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch.is_numeric() {
                self.position += 1;
            } else {
                break;
            }
        }
        let number_str: String = (self.input[start..self.position]).iter().collect();
        let number = number_str.parse::<i64>().unwrap();
        Some(Token::Number(number))
    }

    fn read_punctuation(&mut self, c: char) -> Option<Token> {
        self.position += 1;
        Some(Token::Punctuation(c))
    }

    fn peek_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }
        Some(self.input[self.position])
    }

    fn peek_next_char(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            return None;
        }
        Some(self.input[self.position + 1])
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Word(String),
    Number(i64),
    Whitespace,
    Punctuation(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_creation() {
        let input = "Hello, world!";
        let tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.input, input.chars().collect::<Vec<char>>());
        assert_eq!(tokenizer.position, 0);
    }

    #[test]
    fn test_peek_char() {
        let mut tokenizer = Tokenizer::new("abc");
        assert_eq!(tokenizer.peek_char(), Some('a'));
        tokenizer.position += 1;
        assert_eq!(tokenizer.peek_char(), Some('b'));
        tokenizer.position += 1;
        assert_eq!(tokenizer.peek_char(), Some('c'));
        tokenizer.position += 1;
        assert_eq!(tokenizer.peek_char(), None);
    }

    #[test]
    fn test_peek_next_char() {
        let mut tokenizer = Tokenizer::new("abc");
        assert_eq!(tokenizer.peek_next_char(), Some('b'));
        tokenizer.position += 1;
        assert_eq!(tokenizer.peek_next_char(), Some('c'));
        tokenizer.position += 1;
        assert_eq!(tokenizer.peek_next_char(), None);
    }

    #[test]
    fn test_simple_tokenization() {
        let mut tokenizer = Tokenizer::new("Hello, World!");
        assert_eq!(0, tokenizer.position);
        assert_tokens(
            &mut tokenizer,
            &[
                Token::Word("Hello".to_string()),
                Token::Punctuation(','),
                Token::Whitespace,
                Token::Word("World".to_string()),
                Token::Punctuation('!'),
            ],
        );
        assert_eq!(13, tokenizer.position);
    }

    #[cfg(feature = "count-apostrophed-words-as-one")]
    #[test]
    fn test_tokenize_word_with_apostrophe() {
        let mut tokenizer = Tokenizer::new("It's a friggin' 'test' that would've passed.");
        assert_tokens(
            &mut tokenizer,
            &[
                Token::Word("It's".to_string()),
                Token::Whitespace,
                Token::Word("a".to_string()),
                Token::Whitespace,
                Token::Word("friggin".to_string()),
                Token::Punctuation('\''),
                Token::Whitespace,
                Token::Punctuation('\''),
                Token::Word("test".to_string()),
                Token::Punctuation('\''),
                Token::Whitespace,
                Token::Word("that".to_string()),
                Token::Whitespace,
                Token::Word("would've".to_string()),
                Token::Whitespace,
                Token::Word("passed".to_string()),
                Token::Punctuation('.'),
            ],
        );
    }

    #[cfg(not(feature = "count-apostrophed-words-as-one"))]
    #[test]
    fn test_tokenize_word_with_apostrophe() {
        let mut tokenizer = Tokenizer::new("It's a friggin' 'test' that would've passed.");
        assert_tokens(
            &mut tokenizer,
            &[
                Token::Word("It".to_string()),
                Token::Punctuation('\''),
                Token::Word("s".to_string()),
                Token::Whitespace,
                Token::Word("a".to_string()),
                Token::Whitespace,
                Token::Word("friggin".to_string()),
                Token::Punctuation('\''),
                Token::Whitespace,
                Token::Punctuation('\''),
                Token::Word("test".to_string()),
                Token::Punctuation('\''),
                Token::Whitespace,
                Token::Word("that".to_string()),
                Token::Whitespace,
                Token::Word("would".to_string()),
                Token::Punctuation('\''),
                Token::Word("ve".to_string()),
                Token::Whitespace,
                Token::Word("passed".to_string()),
                Token::Punctuation('.'),
            ],
        );
    }

    fn assert_tokens(tokenizer: &mut Tokenizer, tokens: &[Token]) {
        for token in tokens {
            assert_eq!(tokenizer.next_token(), Some(token.clone()));
        }
        assert_eq!(tokenizer.next_token(), None);
    }
}
