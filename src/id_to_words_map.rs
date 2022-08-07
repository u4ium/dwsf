use std::{collections::BTreeMap, ops::Deref};

use crate::word_id::WordId;

pub struct IdToWordsMap<'a>(BTreeMap<WordId, Vec<&'a str>>);

impl<'a> FromIterator<&'a str> for IdToWordsMap<'a> {
    fn from_iter<I>(words: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        Self(words.into_iter().fold(BTreeMap::new(), |mut m, word| {
            m.entry(WordId::from(word)).or_default().push(word);
            m
        }))
    }
}

impl<'a> Deref for IdToWordsMap<'a> {
    type Target = BTreeMap<WordId, Vec<&'a str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
