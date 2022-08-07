// each word fits into a u32 BitSet?

use std::{fmt::Display, ops::Deref};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WordId(u32);

impl From<&str> for WordId {
    fn from(word: &str) -> Self {
        Self(
            word.as_bytes()
                .into_iter()
                .fold(0, |a, c| a | 1 << (c.to_ascii_lowercase() - 'a' as u8)),
        )
    }
}

impl Deref for WordId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for WordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..32)
                .map(|i| {
                    if self.0 & 1 << i != 0 {
                        Some(('a' as u8 + i) as char)
                    } else {
                        None
                    }
                })
                .flatten()
                .join("")
        )
    }
}
