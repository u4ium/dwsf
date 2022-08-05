//TODO: is this file needed?

use std::{collections::BTreeMap, ops::Deref};

//TODO: hierarchy
use crate::word_id::WordId;

pub struct IdToWordsMap<'a>(BTreeMap<WordId, Vec<&'a str>>);

impl<'a> FromIterator<(WordId, &'a str)> for IdToWordsMap<'a> {
    fn from_iter<I>(tuples: I) -> Self
    where
        I: IntoIterator<Item = (WordId, &'a str)>,
    {
        Self(tuples.into_iter().fold(BTreeMap::new(), |mut m, (k, v)| {
            m.entry(k).or_default().push(v);
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
