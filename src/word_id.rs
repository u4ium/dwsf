// each word fits into a u32 BitSet?

use std::ops::Deref;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
