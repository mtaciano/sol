mod tokens;

use std::iter::Iterator;

use tokens::Token;

pub struct Lexer {
    input: Vec<char>,
    idx: usize,
    line: usize,
    column: usize,
    first_token: bool,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            idx: 0,
            line: 1,
            column: 1,
            first_token: true,
        }
    }

    pub fn read_token(&mut self) -> Token {
        self.consume_whitespace();

        match self.next() {
            Some('=') => {
                if self.peek() == Some('=') {
                    self.consume();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek() == Some('=') {
                    self.consume();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some('*') => Token::Asterisk,
            Some('/') => {
                if self.peek() == Some('*') {
                    self.consume_comment();
                    // Since the lexer ignores comments we need to call `read_token()` again
                    self.read_token()
                } else {
                    Token::Slash
                }
            }
            Some('<') => {
                if self.peek() == Some('=') {
                    self.consume();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            Some('>') => {
                if self.peek() == Some('=') {
                    self.consume();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,
            Some(ch) if ch.is_alphabetic() => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Fn,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "while" => Token::While,
                    "for" => Token::For,
                    _ => Token::Ident(ident),
                }
            }
            Some(ch) if ch.is_numeric() => {
                let number = self.read_number();

                match number {
                    Ok(num) => Token::Integer(num),
                    Err(_) => Token::Invalid,
                }
            }
            Some(ch) => {
                if ch.is_whitespace() {
                    self.consume_whitespace();
                    self.read_token()
                } else {
                    Token::Invalid
                }
            }
            None => Token::Eol,
        }
    }

    fn read_number(&mut self) -> Result<i32, std::num::ParseIntError> {
        let mut number = String::from(
            self.current()
                .expect("Should be called only after read_token"),
        );

        while let Some(peek_ch) = self.peek() {
            if !peek_ch.is_numeric() {
                break;
            }

            number.push(self.next().expect("The char was peeked"));
        }

        number.parse()
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::from(
            self.current()
                .expect("Should be called only after read_token"),
        );

        while let Some(peek_ch) = self.peek() {
            if !peek_ch.is_alphanumeric() && peek_ch != '_' {
                break;
            }

            ident.push(self.next().expect("The char was peeked"));
        }

        ident
    }

    fn current(&mut self) -> Option<char> {
        if self.idx >= self.input.len() {
            return None;
        }

        Some(self.input[self.idx])
    }

    fn next(&mut self) -> Option<char> {
        if !self.first_token {
            self.idx += 1;
        }
        self.first_token = false;

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

    fn peek(&self) -> Option<char> {
        if self.idx + 1 >= self.input.len() {
            return None;
        }

        Some(self.input[self.idx + 1])
    }

    fn consume(&mut self) {
        let _ = self.next();
    }

    fn consume_comment(&mut self) {
        if self.current() != Some('/') && self.peek() != Some('*') {
            return;
        }

        while let Some(c) = self.next() {
            if c == '*' && self.peek() == Some('/') {
                /* Comments are like this in sol */
                self.consume();
                break;
            }
        }
    }

    fn consume_whitespace(&mut self) {
        if let Some(ch) = self.current()
            && !ch.is_whitespace()
        {
            return;
        }

        while self.peek().is_some() && self.peek().unwrap().is_whitespace() {
            self.consume();
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.read_token();

        // The lexer will return EOL when the index becomes bigger than the vec size
        if tok == Token::Eol {
            return None;
        }

        Some(tok)
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
        println!("asdf");

        assert_eq!(expected_tokens, real_tokens);
    }
}
