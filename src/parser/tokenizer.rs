use std::io::{Bytes, Read};
use std::iter::Peekable;
use std::fmt;
use super::char_helpers::*;
use super::Chars::*;

pub enum TokenError {
    Read,
    Parse {line_num: i32, column_num: i32, character: char },
    Cast {line_num: i32, column_num: i32, string: String },
}

impl fmt::Debug for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::Read => {
                write!(f, "Error when reading next token.  Unusual!")
            },
            TokenError::Parse { line_num, column_num, character } => {
                write!(f, "Found '{}' at {}, {}", character, line_num, column_num)
            },
            TokenError::Cast { line_num, column_num, ref string } => {
                write!(f, "Unable to cast \"{}\" at {}, {}", string, line_num, column_num)
            }
        }         
    }
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Float(f32),
    Int(i32),
    Comma,
    WhiteSpace,
}

pub type NextToken = Result<Token, TokenError>;

pub struct Tokenizer <T:Read> {
    chars: Peekable<Chars<T>>,
    line_num: i32,
    column_num: i32,
}

impl <T:Read> Tokenizer <T> {
    pub fn from_reader(reader: T) -> Tokenizer<T> {
        Tokenizer { 
            chars: Chars::new(reader).peekable(),
            line_num: 0,
            column_num: 0
        }
    }

    fn advance<F>(&mut self, mut token_string: String, f: F) -> Option<NextToken> 
        where F:Fn(&mut Self, char, String) -> Option<NextToken> {
        match self.chars.next() {
            None    => return Some(Err(TokenError::Read)),
            Some(c) => {
                if is_end_of_line(c) {
                    self.line_num += 1; 
                    self.column_num = 0;
                } else {
                    self.column_num += 1; 
                }
                token_string.push(c);
                f(self, c, token_string)
            }
        }
    }

    fn new_parse_error(&self, character: char) -> TokenError {
        TokenError::Parse {
            line_num: self.line_num,
            column_num: self.column_num,
            character: character,
        }
    }

    fn new_cast_error(&self, string: String) -> TokenError {
        TokenError::Cast {
            line_num: self.line_num,
            column_num: self.column_num,
            string: string
        } 
    }

    fn cast_float(&self, token_string: String) -> NextToken {
        match token_string.parse::<f32>() {
            Ok(t) => Ok(Token::Float(t)),
            _     => Err(self.new_cast_error(token_string))
        }
    }

    fn cast_int(&self, token_string: String) -> NextToken {
        match token_string.parse::<i32>() {
            Ok(t) => Ok(Token::Int(t)),
            _     => Err(self.new_cast_error(token_string))
        }
    }

    fn tokenize_decimal(&mut self, c: char, mut token_string: String) -> Option<NextToken> {
        match self.chars.peek().cloned() {
            Some(c) if c.is_numeric()    => self.advance(token_string, Self::tokenize_decimal),
            Some(c) if c.is_whitespace() => Some(self.cast_float(token_string)),
            Some(',')                    => Some(self.cast_float(token_string)),
            None                         => Some(self.cast_float(token_string)),
            Some(c)                      => Some(Err(self.new_parse_error(c))),
        } 
    }

    fn tokenize_num(&mut self, c: char, mut token_string: String) -> Option<NextToken> {
        match self.chars.peek().cloned() {
            Some(c) if c.is_numeric()    => self.advance(token_string, Self::tokenize_num),
            Some('.')                    => self.advance(token_string, Self::tokenize_decimal),
            Some(c) if c.is_whitespace() => Some(self.cast_int(token_string)),
            Some(',')                    => Some(self.cast_int(token_string)),
            None                         => Some(self.cast_int(token_string)),
            Some(c)                      => Some(Err(self.new_parse_error(c))),
        }
    }

    fn tokenize_identifier(&mut self, c: char, mut token_string: String) -> Option<NextToken> {
        match self.chars.peek().cloned() {
            Some(c) if c.is_alphanumeric() => self.advance(token_string, Self::tokenize_identifier),
            Some(c) if c.is_whitespace()   => Some(Ok(Token::Identifier(token_string))),
            None                           => Some(Ok(Token::Identifier(token_string))),
            Some(c)                        => Some(Err(self.new_parse_error(c))),
        }
    }

    fn tokenize_whitespace(&mut self, c: char, mut token_string: String) -> Option<NextToken> {
        match self.chars.peek().cloned() {
            Some(c) if c.is_whitespace() => self.advance(token_string, Tokenizer::<T>::tokenize_whitespace),
            _                            => Some(Ok(Token::WhiteSpace)),
        }
    }

    fn tokenize_punctuation(&mut self, c: char, mut token_string: String) -> Option<NextToken> {
        match c {
            ',' => Some(Ok(Token::Comma)),
            _   => Some(Err(self.new_parse_error(c))),
        }
    }
}

impl <T:Read> Iterator for Tokenizer<T> {
    type Item = NextToken;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token_string = String::new();

        match self.chars.peek().cloned() {
            Some(c @ '-')                  => self.advance(token_string, Self::tokenize_num),
            Some(c) if c.is_numeric()      => self.advance(token_string, Self::tokenize_num),
            Some(c) if c.is_alphanumeric() => self.advance(token_string, Self::tokenize_identifier),
            Some(c) if c.is_whitespace()   => self.advance(token_string, Self::tokenize_whitespace),
            Some(',')                      => self.advance(token_string, Self::tokenize_punctuation),
            _                              => None
        }
    }
}
