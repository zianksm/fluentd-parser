use core::convert::TryFrom;

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
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct $ident(pub(crate) $tt);
        impl_type_state!($ident);
    };
}

impl_type_state!(state = ArbitraryIdent,inner type = String);
impl_type_state!(state = ArbitraryArgs,inner type = String);
impl_type_state!(state = Events,inner type = String);
impl_type_state!(state = PortNumber,inner type = String);

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
    Port(PortNumber),

    Source,
    Match(Events),
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
