use std::collections::BTreeSet; // TODO: remove

mod word_id;
use word_id::*;
mod world_graph;
use world_graph::*;
mod id_to_word_map;
use id_to_word_map::*;

fn find_N_cliques<const N: usize>(graph: WordGraph) -> Vec<[WordId; N]> {
    todo!("find cliques within this graph of size N")
}

/// TODO: generalize over 5,5
pub fn wordle_55(words: Vec<&str>) -> BTreeSet<[String; 5]> {
    // TODO: replace with id to words
    let word_map = IdToWordMap::new(words);
    let graph = WordGraph::new(word_map.keys().cloned());
    let x = find_N_cliques::<5>(graph);

    BTreeSet::new()
}

//TODO: move tests to submodules
#[cfg(test)]
mod tests {
    use crate::{wordle_55, IdToWordMap, WordGraph, WordId};

    #[test]
    fn wordle_has_538_cliques_of_disjoint_words() {
        let words = wordle_55(vec![]);
        assert_eq!(
            words.len(),
            538,
            "Matt Parker is a better programmer than I 😢"
        );
    }

    #[test]
    fn word_ids() {
        assert_eq!(*WordId::from("aaaaa"), 0b000000_00001);
        assert_eq!(*WordId::from("abcde"), 0b00000_11111);
        assert_eq!(*WordId::from("abcdf"), 0b00001_01111);
    }

    #[test]
    fn word_graphs() {
        let test_words = vec![
            "abcde",
            // 0b0_00000_00000_00000_00000_11111
            // "abcde" -> ["zlmno"]
            "awxyz",
            // 0b1_11100_00000_00000_00000_00001
            // "awxyz" -> []
            "zlmno",
            // 0b1_00000_00000_01110_00000_00000
            // "zlmno" -> ["abcde"]
        ];
        let id_to_word_map = IdToWordMap::new(test_words);
        let word_graph = WordGraph::new(id_to_word_map.keys().cloned());

        assert_eq!(word_graph["abcde"].len(), 1, "abcde");
        assert_eq!(word_graph["awxyz"].len(), 0, "awxyz");
        assert_eq!(word_graph["zlmno"].len(), 1, "zlmno");
    }
}
