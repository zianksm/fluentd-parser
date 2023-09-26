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

impl_type_state!(state = Literal,inner type = String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Dot,
    Newline,
    Whitespace,
    Indetifier(Literal),
    LeftAngle,
    RightAngle,
    // "@"
    AtSign,
    HashTag,
    Quote,
    ForwardSlash,
    // "port"
    Port,
    Source,
    Match,
    Filter,
    System,
    Label,
    Worker,
}

impl Token {
    pub fn is_non_ident(str: &str) -> bool {
        match str {
            "." | "\n" | " " | "<" | ">" | "@" | "#" | "\"" | "/" => true,
            _ => false,
        }
    }
}
