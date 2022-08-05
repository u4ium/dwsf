//TODO: hierarchy
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Index,
};

use crate::word_id::WordId;

pub struct WordGraph(BTreeMap<WordId, BTreeSet<WordId>>);

impl WordGraph {
    // TODO: from
    pub fn new(word_ids: impl Iterator<Item = WordId> + Clone) -> Self {
        //TODO: optimize: a -> b iff b -> a (x2)
        Self(
            word_ids
                .clone() //TODO: don't clone
                .map(|word_id| {
                    (
                        word_id,
                        word_ids
                            .clone() //TODO: don't clone
                            .filter(|&other_word_id| *other_word_id & *word_id == 0)
                            .collect(),
                    )
                })
                .collect(),
        )
    }
}

impl Index<&str> for WordGraph {
    type Output = BTreeSet<WordId>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[&WordId::from(index)]
    }
}
