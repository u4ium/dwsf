#![feature(map_first_last)]

use itertools::Itertools;

use std::collections::BTreeSet; // TODO: remove

mod word_id;
use word_id::*;
mod word_graph;
use word_graph::*;
mod id_to_words_map;
use id_to_words_map::*;

fn find_cliques<const N: usize>(word_ids: &Vec<WordId>) -> Vec<[WordId; N]> {
    let mut ret: Vec<BTreeSet<WordId>> = vec![];

    for _ in 1..=5 {
        ret = ret
            .into_iter()
            .map(|set| {
                word_ids
                    .iter()
                    .filter_map(|key| {
                        if key > set.last().unwrap_or(&WordId::default())
                            && !set.iter().any(|word_id| (**key & **word_id) != 0)
                        {
                            let mut y = set.clone();
                            y.insert(*key);
                            Some(y)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
    }

    ret.into_iter()
        .map(|set| {
            set.into_iter()
                .collect::<Vec<_>>()
                .try_into()
                .expect("must_have {N}")
        })
        .collect()
}

fn find_cliques_of_size_n<const N: usize>(graph: WordGraph) -> Vec<[WordId; N]> {
    let mut ret: Vec<BTreeSet<WordId>> = graph.keys().map(|_k| BTreeSet::new()).collect();

    for _ in 1..=5 {
        ret = ret
            .into_iter()
            .map(|set| {
                graph
                    .keys()
                    .filter_map(|key| {
                        if key > set.last().unwrap_or(&WordId::default())
                            && set.is_subset(graph.get(key).expect("graph has key"))
                        {
                            let mut y = set.clone();
                            y.insert(*key);
                            Some(y)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
    }

    ret.into_iter()
        .map(|set| {
            set.into_iter()
                .collect::<Vec<_>>()
                .try_into()
                .expect("must_have {N}")
        })
        .collect()
}

// TODO: generalize over N,M
// TODO: rename
pub fn wordle_55<'a>(
    words: impl IntoIterator<Item = &'a str>,
    //TODO: -> impl IntoIterator<Item = [&'a str; 5]>
) -> impl IntoIterator<Item = [String; 5]> {
    // TODO: replace with id to words
    let word_map = IdToWordsMap::from_iter(words);
    let cliques = find_cliques::<5>(&word_map.keys().into_iter().cloned().collect());
    construct_result(word_map, cliques)
}

// TODO: test
// TODO: rename
// TODO: needs M=5? (string length)
fn construct_result<const N: usize>(
    word_map: IdToWordsMap,
    cliques: Vec<[WordId; N]>,
) -> BTreeSet<[String; N]> {
    BTreeSet::from_iter(
        cliques
            .iter()
            .map(|clique| {
                clique
                    .into_iter()
                    .flat_map(|cq| &word_map[cq])
                    .map(|&s| s.to_owned())
                    .combinations(N)
                    .map(|c| c.try_into().expect("must have {N}"))
                    .collect::<Vec<_>>()
            })
            .flatten(),
    )
}

// TODO: move tests to submodules
#[cfg(test)]
mod tests {

    use std::collections::BTreeSet;

    use crate::{id_to_words_map::IdToWordsMap, wordle_55, WordGraph, WordId};

    #[test]
    fn wordle_has_538_cliques_of_disjoint_words() {
        use std::fs;

        let foo = fs::read_to_string("res/words.txt").unwrap();
        let words: Vec<_> = foo
            .split(char::is_whitespace)
            .filter(|&word| BTreeSet::from_iter(word.as_bytes()).len() == 5)
            .collect();

        // let wordle_words = wordle_55(words).into_iter().collect::<Vec<_>>();

        // assert_eq!(
        //     wordle_words.len(),
        //     538,
        //     "Matt Parker is a better programmer than I 😢"
        // );
    }

    #[test]
    fn word_id() {
        assert_eq!(*WordId::from("aaaaa"), 0b000000_00001);
        assert_eq!(*WordId::from("abcde"), 0b00000_11111);
        assert_eq!(*WordId::from("abcdf"), 0b00001_01111);
    }

    #[test]
    fn word_graph_simple() {
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

    mod single_5_clique {
        use super::*;

        const WORDS: [&'static str; 5] = ["abcde", "fghij", "klmno", "pqrst", "uvwxy"];

        #[test]
        fn word_graph_with_5_clique() {
            let id_to_words_map = IdToWordsMap::from_iter(WORDS);
            let word_graph = WordGraph::new(id_to_words_map.keys().cloned());

            assert_eq!(word_graph["abcde"].len(), 4, "abcde");
            assert_eq!(word_graph["fghij"].len(), 4, "fghij");
            assert_eq!(word_graph["klmno"].len(), 4, "klmno");
            assert_eq!(word_graph["pqrst"].len(), 4, "pqrst");
            assert_eq!(word_graph["uvwxy"].len(), 4, "uvwxy");

            let cliques: Vec<_> = wordle_55(WORDS).into_iter().collect();
            assert_eq!(cliques.len(), 1)
        }
    }
}
