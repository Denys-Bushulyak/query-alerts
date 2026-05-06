use std::fmt::Display;

pub const LANGUAGE_CODE_LEN: usize = 2;

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct Language([u8; LANGUAGE_CODE_LEN]);

impl From<[u8; LANGUAGE_CODE_LEN]> for Language {
    fn from(array: [u8; LANGUAGE_CODE_LEN]) -> Self {
        Language(array)
    }
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        Language(s.as_bytes()[..LANGUAGE_CODE_LEN].try_into().unwrap())
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0).to_string())
    }
}
