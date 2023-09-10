use core::convert::TryFrom;

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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AtSignIdent {
    Type(String),
    Include(String),
}

impl AtSignIdent {
    pub fn from_str_with_ident(ident: String, args: String) -> Result<Self, String> {
        match ident.as_str() {
            "type" => Ok(Self::Type(args.to_string())),
            "include" => Ok(Self::Include(args.to_string())),
            _ => Err(format!("Unknown @ sign identifier: {}", ident)),
        }
    }
}
