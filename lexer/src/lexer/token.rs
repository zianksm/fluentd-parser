use core::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ArbitraryIdent(pub(crate) String);

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ArbitraryArgs(pub(crate) String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexError {
    InvalidToken(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LeftAngle,
    RightAngle,
    // "@"
    AtSign(AtSignIdent),

    // "#" for comments
    HashTag(String),

    Quote,
    ForwardSlash,

    // "port"
    Port(u16),

    Source,
    Match,
    Filter,
    System,
    Label,
    Worker,
    Ident(ArbitraryIdent, ArbitraryArgs),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AtSignIdent {
    Type(String),
    Include(String),
}

impl AtSignIdent {
    pub fn from_str_with_ident(ident: String, args: String) -> Result<Self, LexError> {
        match ident.as_str() {
            "type" => Ok(Self::Type(args.trim().to_string())),
            "include" => Ok(Self::Include(args.trim().to_string())),
            _ => Err(LexError::InvalidToken(ident)),
        }
    }
}
