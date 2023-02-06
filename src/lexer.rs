mod tokens;

use tokens::Token;

/// The sol language lexer
pub struct Lexer {
    input: Vec<char>,
    current_idx: isize, // TODO: find a good way to use usize
    peek_idx: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        assert!(input.len() >= 2); // We have to at least be able to call `current()` and `peek()`

        Lexer {
            input: input.chars().collect(),
            current_idx: -1, // HACK: start with an offset so `next()` works properly
            peek_idx: 1,
            line: 1,
            column: 1,
        }
    }

    pub fn read_token(&mut self) -> Token {
        self.consume_whitespace();

        match self.next() {
            Some('=') => {
                if self.peek() == Some('=') {
                    let _ = self.next();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek() == Some('=') {
                    let _ = self.next();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some('*') => Token::Asterisk,
            Some('/') => {
                if self.peek() == Some('*') {
                    self.consume_comment();
                    // Since the lexer ignores comments it needs to call `read_token()` again
                    self.read_token()
                } else {
                    Token::Slash
                }
            }
            Some('<') => {
                if self.peek() == Some('=') {
                    let _ = self.next();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            Some('>') => {
                if self.peek() == Some('=') {
                    let _ = self.next();
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
        // We can unwrap safely here because `read_token()` proved that it isn't None
        let mut number = String::from(self.current().unwrap());

        while let Some(peek_ch) = self.peek() {
            if !peek_ch.is_numeric() {
                break;
            }

            number.push(self.next().unwrap());
        }

        number.parse()
    }

    fn read_identifier(&mut self) -> String {
        // We can unwrap safely here because `read_token()` proved that it isn't None
        let mut ident = String::from(self.current().unwrap());

        while let Some(peek_ch) = self.peek() {
            if !peek_ch.is_alphanumeric() && peek_ch != '_' {
                break;
            }

            ident.push(self.next().unwrap());
        }

        ident
    }

    fn current(&mut self) -> Option<char> {
        // We assume that if it fails it's because it's -1
        // The case when it overflows from the top (x > usize::MAX) is not handled
        let idx = usize::try_from(self.current_idx).unwrap_or(0);
        if idx >= self.input.len() {
            return None;
        }

        Some(self.input[idx])
    }

    fn next(&mut self) -> Option<char> {
        if (self.current_idx + 1) as usize >= self.input.len() {
            return None;
        }

        // We increment first, and then get the caracter, since that way everytime `next()`
        // is called, the value of `self.current_idx` will be at the current char
        self.current_idx += 1;
        if self.current_idx != 0 {
            // Will only not happen once
            self.peek_idx += 1;
        }

        let ch = self.input[self.current_idx as usize];

        self.column += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 0;
        }

        Some(ch)
    }

    fn peek(&self) -> Option<char> {
        if self.peek_idx >= self.input.len() {
            return None;
        }

        Some(self.input[self.peek_idx])
    }

    fn consume_comment(&mut self) {
        // Do not start the process if it's not indeed a comment
        if self.current() != Some('/') && self.peek() != Some('*') {
            return;
        }

        while let Some(c) = self.next() {
            if c == '*' && self.peek() == Some('/') {
                /* Comments are like this in sol */
                // We call `next()` because `peek()` does not increment the index
                let _ = self.next();
                break;
            }
        }
    }

    fn consume_whitespace(&mut self) {
        // Do not start the process if it's not indeed whitespace
        if let Some(ch) = self.current() && !ch.is_whitespace() {
            return;
        }

        while self.peek().is_some() && self.peek().unwrap().is_whitespace() {
            let _ = self.next();
        }
    }
}

// TODO: it seems that even when the interior state mutates
// rust doesn't require that you use `let mut` when using `for tk in lexer`, maybe this is a bug?
impl std::iter::Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tk = self.read_token();

        // The lexer will return EOL when the index becomes bigger than the vec size
        if tk == Token::Eol {
            return None;
        }

        Some(tk)
    }
}

#[cfg(test)]
mod tests {
    use super::tokens::Token;
    use super::Lexer;

    #[test]
    fn create_lexer() {
        // TODO: verify this test usefulness
        let input = String::from("test\nstring\táéíóú");
        let _ = Lexer::new(input);
    }

    #[test]
    fn read_number_valid() {
        let input = String::from("12345  ");
        let mut lexer = Lexer::new(input);
        let _ = lexer.next(); // Simulate the `read_token()` behaviour

        assert_eq!(lexer.read_number(), Ok(12345));
    }

    #[test]
    fn read_number_after_whitespace() {
        let input = String::from("  \t\r 12345");
        let mut lexer = Lexer::new(input);

        lexer.consume_whitespace();
        let _ = lexer.next(); // Simulate the `read_token()` behaviour

        assert_eq!(lexer.read_number(), Ok(12345));
    }

    #[test]
    #[should_panic]
    fn read_number_invalid() {
        let input = String::from("ABC");
        let mut lexer = Lexer::new(input);

        lexer.read_number().unwrap();
    }

    #[test]
    fn read_ident() {
        let input = String::from("identifier   ");
        let mut lexer = Lexer::new(input);
        let _ = lexer.next(); // Simulate the `read_token()` behaviour

        assert_eq!(lexer.read_identifier().as_str(), "identifier");
    }

    #[test]
    fn read_ident_after_whitespace() {
        let input = String::from("  \t\r identifier");
        let mut lexer = Lexer::new(input);

        lexer.consume_whitespace();
        let _ = lexer.next(); // Simulate the `read_token()` behaviour

        assert_eq!(lexer.read_identifier().as_str(), "identifier");
    }

    #[test]
    fn read_token_declaration() {
        let input = String::from("let ident;");
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        for tk in lexer {
            tokens.push(tk);
        }

        // TODO: create a macro
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Ident("ident".to_string()),
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn read_token_assign() {
        let input = String::from("ident = 42;");
        let lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        for tk in lexer {
            tokens.push(tk);
        }

        // TODO: create a macro
        assert_eq!(
            tokens,
            vec![
                Token::Ident("ident".to_string()),
                Token::Assign,
                Token::Integer(42),
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn current() {
        let input = String::from("AB");
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.current(), Some('A'));
    }

    #[test]
    fn next() {
        let input = String::from("this");
        let mut lexer = Lexer::new(input);
        let mut output = Vec::new();

        while let Some(c) = lexer.next() {
            output.push(c);
        }

        assert_eq!(output, vec!['t', 'h', 'i', 's']);
    }

    #[test]
    fn peek() {
        let input = String::from("AB");
        let lexer = Lexer::new(input);

        assert_eq!(lexer.peek(), Some('B'));
    }

    #[test]
    fn consume_comment() {
        let input = String::from("/* asdf asdf asdf */A");
        let mut lexer = Lexer::new(input);

        lexer.consume_comment();
        assert_eq!(lexer.next(), Some('A'));
    }

    #[test]
    fn consume_comment_nothing_to_consume() {
        let input = String::from("ABC");
        let mut lexer = Lexer::new(input);

        lexer.consume_comment();
        assert_eq!(lexer.next(), Some('A'));
    }

    #[test]
    fn consume_whitespace() {
        let input = String::from("   \t\t  \n\r\n A");
        let mut lexer = Lexer::new(input);

        lexer.consume_whitespace();
        assert_eq!(lexer.next(), Some('A'));
    }

    #[test]
    fn consume_whitespace_nothing_to_consume() {
        let input = String::from("ABC");
        let mut lexer = Lexer::new(input);

        lexer.consume_whitespace();
        assert_eq!(lexer.next(), Some('A'));
    }
}
