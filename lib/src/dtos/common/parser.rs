use serde::{Deserialize, Deserializer};

use crate::entities::{LANGUAGE_CODE_LEN, Language};

pub fn string_to_u8_array<'de, D>(deserializer: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    let bytes = s.as_bytes();
    if bytes.len() != LANGUAGE_CODE_LEN {
        return Err(serde::de::Error::custom(format!(
            "expected a string of length 2, got {}",
            bytes.len()
        )));
    }

    let mut array = [0u8; LANGUAGE_CODE_LEN];
    array.copy_from_slice(bytes);

    Ok(array.into())
}
