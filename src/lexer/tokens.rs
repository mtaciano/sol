//! The tokens used by the lexer for the sol language.
//!
//! This module contains all implementations related to the tokens for the language, such as how
//! they are displayed, their names and what tokens exist.

use std::fmt;

type Name = String;

/// The token representation of the sol language.
///
/// This is the list of all the possible tokens in the sol language, any string containing source
/// code for the sol language should be decomposable into a list of these tokens.
#[derive(PartialEq, Debug)]
pub enum Token {
    /// Invalid input.
    ///
    /// An invalid input consists of either characters not present in the sol language (e.g. `~`, `:`)
    /// or invalid combinations of characters (e.g. `42foo`).
    Invalid,

    /// String identifier.
    Ident(Name),

    /// Integer literal (i.e. a number).
    Integer(i32),

    /// Assign operator (i.e. `=`).
    Assign,
    /// Plus operator (i.e. `+`).
    Plus,
    /// Minus operator (i.e. `-`).
    Minus,
    /// Negation operator (i.e. `!`).
    Bang,
    /// Multiplication operator (i.e. `*`).
    Asterisk,
    /// Division operator (i.e. `/`).
    Slash,
    /// Less than operator (i.e. `<`).
    Lt,
    /// Less than or equal operator (i.e. `<=`).
    LtEq,
    /// Greater than operator (i.e. `>`).
    Gt,
    /// Greater than or equal operator (i.e. `>=`).
    GtEq,
    /// Equal operator (i.e. `==`).
    Eq,
    /// Not equal operator (i.e. `!=`).
    NotEq,

    /// Comma delimiter (i.e. `,`).
    Comma,
    /// Semicolon delimiter (i.e. `;`).
    Semicolon,
    /// Left parenthesis delimiter (i.e. `(`).
    LParen,
    /// Right parenthesis delimiter (i.e. `)`).
    RParen,
    /// Left brace delimiter (i.e. `{`).
    LBrace,
    /// Right brace delimiter (i.e. `}`).
    RBrace,
    /// Left square bracket delimiter (i.e. `[`).
    LBracket,
    /// Right square bracket delimiter (i.e. `]`).
    RBracket,

    /// `decl` keyword.
    Decl,
    /// `while` keyword.
    While,
    /// `for` keyword.
    For,
    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `return` keyword.
    Return,
    /// `fun` keyword.
    Fun,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Invalid => write!(f, "Invalid"),
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
            Token::Decl => write!(f, "decl"),
            Token::While => write!(f, "while"),
            Token::For => write!(f, "for"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Fun => write!(f, "fun"),
        }
    }
}
