mod tokens;

use anyhow::{self, Context};
use std::iter::Iterator;

use tokens::Token;

pub struct Lexer {
    input: Vec<char>,
    idx: usize,
    line: usize,
    column: usize,
    is_first_token: bool,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            idx: 0,
            line: 1,
            column: 1,
            is_first_token: true,
        }
    }

    pub fn read_token(&mut self) -> Option<Token> {
        self.consume_whitespace();

        match self.next_char() {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.consume_char();
                    Some(Token::Eq)
                } else {
                    Some(Token::Assign)
                }
            }
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.consume_char();
                    Some(Token::NotEq)
                } else {
                    Some(Token::Bang)
                }
            }
            Some('*') => Some(Token::Asterisk),
            Some('/') => {
                if self.peek_char() == Some('*') {
                    self.consume_block_comment();
                } else if self.peek_char() == Some('/') {
                    self.consume_line_comment();
                } else {
                    return Some(Token::Slash);
                }

                // Since the lexer ignores comments we need to call `read_token()` again
                self.read_token()
            }
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.consume_char();
                    Some(Token::LtEq)
                } else {
                    Some(Token::Lt)
                }
            }
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.consume_char();
                    Some(Token::GtEq)
                } else {
                    Some(Token::Gt)
                }
            }
            Some(',') => Some(Token::Comma),
            Some(';') => Some(Token::Semicolon),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            Some('{') => Some(Token::LBrace),
            Some('}') => Some(Token::RBrace),
            Some('[') => Some(Token::LBracket),
            Some(']') => Some(Token::RBracket),
            Some(ch) if ch.is_alphabetic() => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "let" => Some(Token::Let),
                    "fn" => Some(Token::Fn),
                    "if" => Some(Token::If),
                    "else" => Some(Token::Else),
                    "return" => Some(Token::Return),
                    "while" => Some(Token::While),
                    "for" => Some(Token::For),
                    _ => Some(Token::Ident(ident)),
                }
            }
            Some(ch) if ch.is_numeric() => {
                let number = self.read_number();

                match number {
                    Ok(num) => Some(Token::Integer(num)),
                    Err(_) => Some(Token::Invalid),
                }
            }
            Some(ch) => {
                if ch.is_whitespace() {
                    self.consume_whitespace();
                    self.read_token()
                } else {
                    Some(Token::Invalid)
                }
            }
            None => None,
        }
    }

    fn read_number(&mut self) -> anyhow::Result<i32> {
        let mut number =
            String::from(self.current_char().expect(
                "Should be called only after read_token confirmed at least 1 char is valid",
            ));

        while let Some(peek_ch) = self.peek_char() {
            if !peek_ch.is_numeric() {
                break;
            }

            number.push(self.next_char().expect("The char was peeked"));
        }

        let number = number.parse().context("read number token")?;

        Ok(number)
    }

    fn read_identifier(&mut self) -> String {
        let mut ident =
            String::from(self.current_char().expect(
                "Should be called only after read_token confirmed at least 1 char is valid",
            ));

        while let Some(peek_ch) = self.peek_char() {
            if !peek_ch.is_alphanumeric() && peek_ch != '_' {
                break;
            }

            ident.push(self.next_char().expect("The char was peeked"));
        }

        ident
    }

    fn current_char(&mut self) -> Option<char> {
        if self.idx >= self.input.len() {
            return None;
        }

        Some(self.input[self.idx])
    }

    fn next_char(&mut self) -> Option<char> {
        if !self.is_first_token {
            self.idx += 1;
        }
        self.is_first_token = false;

        if self.idx >= self.input.len() {
            return None;
        }

        let ch = self.input[self.idx];

        self.column += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        }

        Some(ch)
    }

    fn peek_char(&self) -> Option<char> {
        if self.idx + 1 >= self.input.len() {
            return None;
        }

        Some(self.input[self.idx + 1])
    }

    fn consume_char(&mut self) {
        let _ = self.next_char();
    }

    fn consume_block_comment(&mut self) {
        if self.current_char() != Some('/') && self.peek_char() != Some('*') {
            return;
        }

        while let Some(c) = self.next_char() {
            if c == '*' && self.peek_char() == Some('/') {
                /* Comments are like this in sol */
                self.consume_char();
                return;
            }
        }
    }

    fn consume_line_comment(&mut self) {
        if self.current_char() != Some('/') && self.peek_char() != Some('/') {
            return;
        }

        while let Some(c) = self.next_char() {
            if c == '\n' {
                // They can also be like this
                self.consume_char();
                return;
            }
        }
    }

    fn consume_whitespace(&mut self) {
        if let Some(ch) = self.current_char()
            && !ch.is_whitespace()
        {
            return;
        }

        while self.peek_char().is_some_and(char::is_whitespace) {
            self.consume_char();
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_token()
    }
}

#[cfg(test)]
mod tests {
    use super::tokens::Token;
    use super::Lexer;

    #[test]
    fn read_tokens() {
        let input = "=+(){},;";
        let lexer = Lexer::new(input.into());

        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];
        let real_tokens: Vec<Token> = lexer.collect();

        assert_eq!(expected_tokens, real_tokens);
    }

    #[test]
    fn assignments_with_comments() {
        let input = r#"
            let a = 42;
            /* this is a comment */
            let b; /* this is another comment */
            // this is another way of commenting
            b = 99;
        "#;
        let lexer = Lexer::new(input.into());

        let expected_tokens = vec![
            Token::Let,
            Token::Ident(String::from("a")),
            Token::Assign,
            Token::Integer(42),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("b")),
            Token::Semicolon,
            Token::Ident(String::from("b")),
            Token::Assign,
            Token::Integer(99),
            Token::Semicolon,
        ];
        let real_tokens: Vec<Token> = lexer.collect();

        assert_eq!(expected_tokens, real_tokens);
    }
}
