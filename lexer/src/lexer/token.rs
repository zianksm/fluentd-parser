use core::convert::TryFrom;

use serde::{ Deserialize, Serialize };

pub trait TokenTypeStateMarker: ToString + From<String> {}

macro_rules! impl_type_state {
    ($tt:ident) => {
        impl alloc::string::ToString for $tt {
            fn to_string(&self) -> String {
                self.0.clone()
            }
        }

        impl From<String> for $tt {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl TokenTypeStateMarker for $tt {}
    };

    (state = $ident:ident, inner type = $tt:tt) => {
        #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
        pub struct $ident(pub(crate) $tt);
        impl_type_state!($ident);
    };
}

impl_type_state!(state = Literal,inner type = String);

#[cfg(feature = "token")]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Token {
    // "."
    Dot,
    // "\n"
    Newline,
    // " "
    Whitespace,
    // "arbitrary identifier (name, db, etc.)"
    Indetifier(Literal),
    // "<"
    LeftAngle,
    // ">"
    RightAngle,
    // "{"
    LeftCurly,
    // "}"
    RightCurly,
    // "("
    LeftParen,
    // ")"
    RightParen,
    // "["
    LeftBracket,
    // "]"
    RightBracket,
    // ","
    Comma,
    // ";"
    Semicolon,
    // ":"
    Colon,
    // " = "
    Equals,
    // "@"
    AtSign,
    // "#"
    HashTag,
    // " " "
    Quote,
    // "/"
    ForwardSlash,
    //"\"
    BackSlash,
    // "port"
    Port,
    // "source"
    Source,
    // "match"
    Match,
    // "filter"
    Filter,
    // "system"
    System,
    // "label"
    Label,
    // "worker"
    Worker,
    // "record"
    Record,
    // "buffer"
    Buffer,
}


#[cfg(feature = "token")]
impl Token {
    pub fn is_non_ident(str: &char) -> bool {
        match str {
            '.' | '\n' | ' ' | '<' | '>' | '{' | '}' | '(' | ')' | '[' | ']' | ',' | ';' | ':' | '=' | '@' | '#' | '"' | '/' | '\\' => true,
            _ => false,
        }
    }

    pub fn is_ident(str: &char) -> bool {
        !Self::is_non_ident(str)
    }

    pub fn infer_keyword(str: &str) -> Self {
        match str {
            "port" => Self::Port,
            "source" => Self::Source,
            "match" => Self::Match,
            "filter" => Self::Filter,
            "system" => Self::System,
            "label" => Self::Label,
            "worker" => Self::Worker,
            "record" => Self::Record,
            "buffer" => Self::Buffer,
            _ => Self::Indetifier(Literal(String::from(str))),
        }
    }
}
