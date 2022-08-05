//TODO: is this file needed?

use std::{collections::BTreeMap, ops::Deref};

//TODO: hierarchy
use crate::word_id::WordId;

pub struct IdToWordMap<'a>(BTreeMap<WordId, &'a str>);

impl<'a> IdToWordMap<'a> {
    // TODO: from
    pub fn new(words: Vec<&'a str>) -> IdToWordMap {
        Self(
            words
                .into_iter()
                .map(|word| (WordId::from(word), word))
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl<'a> Deref for IdToWordMap<'a> {
    type Target = BTreeMap<WordId, &'a str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
