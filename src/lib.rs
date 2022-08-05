use itertools::Itertools;

use std::collections::BTreeSet; // TODO: remove

mod word_id;
use word_id::*;
mod world_graph;
use world_graph::*;
mod id_to_words_map;
use id_to_words_map::*;

fn find_cliques_of_size_n<const N: usize>(graph: WordGraph) -> Vec<[WordId; N]> {
    todo!("find cliques within this graph of size N")
}

// TODO: generalize over N,M
// TODO: rename
pub fn wordle_55<'a>(
    words: impl IntoIterator<Item = &'a str>,
    //TODO: -> impl IntoIterator<Item = [&'a str; 5]>
) -> impl IntoIterator<Item = [String; 5]> {
    // TODO: replace with id to words
    let word_map = IdToWordsMap::from_iter(words);
    let graph = WordGraph::new(word_map.keys().cloned());
    let cliques = find_cliques_of_size_n::<5>(graph);
    construct_result(word_map, cliques)
}

// TODO: generalize over N
// TODO: test
fn construct_result(word_map: IdToWordsMap, cliques: Vec<[WordId; 5]>) -> BTreeSet<[String; 5]> {
    BTreeSet::from_iter(
        cliques
            .iter()
            .map(|clique| {
                clique
                    .into_iter()
                    .flat_map(|cq| &word_map[cq])
                    .map(|&s| s.to_owned())
                    .combinations(5)
                    .map(|c| c.try_into().unwrap())
                    .collect::<Vec<_>>()
            })
            .flatten(),
    )
}

// TODO: move tests to submodules
#[cfg(test)]
mod tests {
    use crate::{id_to_words_map::IdToWordsMap, wordle_55, WordGraph, WordId};

    #[test]
    fn wordle_has_538_cliques_of_disjoint_words() {
        let words: Vec<&str> = vec![""];
        let wordle_words = wordle_55(words).into_iter().collect::<Vec<_>>();
        assert_eq!(
            wordle_words.len(),
            538,
            "Matt Parker is a better programmer than I 😢"
        );
    }

    #[test]
    fn word_id() {
        assert_eq!(*WordId::from("aaaaa"), 0b000000_00001);
        assert_eq!(*WordId::from("abcde"), 0b00000_11111);
        assert_eq!(*WordId::from("abcdf"), 0b00001_01111);
    }

    #[test]
    fn word_graph() {
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
        let id_to_words_map = IdToWordsMap::from_iter(test_words);
        let word_graph = WordGraph::new(id_to_words_map.keys().cloned());

        assert_eq!(word_graph["abcde"].len(), 1, "abcde");
        assert_eq!(word_graph["awxyz"].len(), 0, "awxyz");
        assert_eq!(word_graph["zlmno"].len(), 1, "zlmno");
    }
}
