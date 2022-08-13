use std::ops::Deref;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_id() {
        assert_eq!(*WordId::from("aaaaa"), 0b000000_00001);
        assert_eq!(*WordId::from("abcde"), 0b00000_11111);
        assert_eq!(*WordId::from("abcdf"), 0b00001_01111);
    }
}
