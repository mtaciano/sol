type Name = String;

#[derive(PartialEq, Debug)]
pub enum Token {
    // Special tokens
    Invalid,
    Eol,

    // Identifier
    Ident(Name),

    // Literals
    Integer(i32),

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /
    Lt,       // <
    LtEq,     // <=
    Gt,       // >
    GtEq,     // >=
    Eq,       // ==
    NotEq,    // !=

    // Delimiters
    Comma,     // ,
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]

    // Keywords
    Let,
    While,
    For,
    If,
    Else,
    Return,
    Fn,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Invalid => write!(f, "Invalid"),
            Token::Eol => write!(f, "EOL"),
            Token::Ident(name) => write!(f, "{name}"),
            Token::Integer(i) => write!(f, "{i}"),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Lt => write!(f, "<"),
            Token::LtEq => write!(f, "<="),
            Token::Gt => write!(f, ">"),
            Token::GtEq => write!(f, ">="),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "["),
            Token::RBrace => write!(f, "]"),
            Token::LBracket => write!(f, "{{"),
            Token::RBracket => write!(f, "}}"),
            Token::Let => write!(f, "let"),
            Token::While => write!(f, "while"),
            Token::For => write!(f, "for"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Fn => write!(f, "fn"),
        }
    }
}
