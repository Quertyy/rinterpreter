use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TokenType {
    #[serde(rename = "[")]
    LeftParen,
    #[serde(rename = "]")]
    RightParen,
    #[serde(rename = "{")]
    LeftBrace,
    #[serde(rename = "}")]
    RightBrace,
    #[serde(rename = ",")]
    Comma,
    #[serde(rename = ".")]
    Dot,
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = ";")]
    Semicolon,
    #[serde(rename = "/")]
    Slash,
    #[serde(rename = "*")]
    Star,
    #[serde(rename = "!")]
    Bang,
    #[serde(rename = "!=")]
    BangEqual,
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = "==")]
    EqualEqual,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = ">=")]
    GreaterEqual,
    #[serde(rename = "-")]
    Less,
    #[serde(rename = "<=")]
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

