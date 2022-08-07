use itertools::Itertools;

use std::collections::BTreeSet; // TODO: remove

mod word_id;
use word_id::*;
mod id_to_words_map;
use id_to_words_map::*;

pub struct CliqueFinder<const N: usize> {
    word_ids: Vec<WordId>,
    result: Vec<[WordId; N]>,
    current: [WordId; N],
}

impl<const N: usize> CliqueFinder<N> {
    pub fn new(word_ids: Vec<WordId>) -> Self {
        Self {
            word_ids,
            result: Default::default(),
            current: [Default::default(); N],
        }
    }

    pub fn search(mut self) -> Vec<[WordId; N]> {
        self.search_helper(0, 0);
        // self.search_helper_2();
        self.result
    }

    fn search_helper_2(&mut self) {
        struct SearchState {
            start_index: usize,
            depth: usize,
        }

        let mut stack: Vec<SearchState> = vec![SearchState {
            start_index: 0,
            depth: 0,
        }];

        while !stack.is_empty() {
            let SearchState { start_index, depth } = stack.pop().expect("The stack is not empty");
            for j in start_index..=self.word_ids.len() - (N - depth) {
                let x = &mut self.current;
                x[depth] = self.word_ids[j];
                if Self::is_clique_of_size(x, depth + 1) {
                    if depth < N - 1 {
                        stack.push(SearchState {
                            start_index: j + 1,
                            depth: depth,
                        });
                        stack.push(SearchState {
                            start_index: j + 1,
                            depth: depth + 1,
                        });
                        break;
                    } else {
                        self.result.push(x.clone());
                    }
                }
            }
        }
    }

    fn search_helper(&mut self, start_index: usize, depth: usize) {
        // Check if any vertices can be inserted
        let rep = self
            .current
            .iter()
            .take(depth)
            .fold(0_u32, |a, &word_id| (a | *word_id));

        for j in start_index..=self.word_ids.len() - (N - depth) {
            if rep & *self.word_ids[j] == 0 {
                self.current[depth] = self.word_ids[j];
                if depth < N - 1 {
                    self.search_helper(j + 1, depth + 1);
                } else {
                    self.result.push(self.current.clone());
                }
            }
        }
    }

    pub fn is_clique(set: &[WordId; N]) -> bool {
        Self::is_clique_of_size(set, N)
    }

    pub fn is_clique_of_size(set: &[WordId; N], n: usize) -> bool {
        set.iter()
            .take(n)
            .combinations(2)
            .all(|combo| **combo[0] & **combo[1] == 0)
    }
}

// TODO: generalize over N,M
// TODO: rename
/// NOTE: Must be 5-letter words
pub fn find_words_with_disjoint_character_sets<'a, const N: usize, const L: u32>(
    words: Vec<&'a str>,
    //TODO: -> Vec<[&'a str; N]>
) -> BTreeSet<[String; N]> {
    let word_map = IdToWordsMap::from_iter(words);
    let cliques = CliqueFinder::new(
        //TODO: move to IdToWordsMap.get_ids_with_n_distinct_letters()
        word_map
            .keys()
            .into_iter()
            .filter(|word_id| word_id.count_ones() == L)
            .cloned()
            .collect(),
    )
    .search();
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
    use std::fs;

    use crate::{find_words_with_disjoint_character_sets, CliqueFinder, WordId};

    #[test]
    fn wordle_has_538_cliques_of_disjoint_words() {
        let file_contents = fs::read_to_string("res/words.txt").unwrap();
        let words: Vec<_> = file_contents.split_whitespace().collect();
        let wordle_words = find_words_with_disjoint_character_sets::<5, 5>(words);

        for word_set in wordle_words.iter() {
            println!("{word_set:?}");
        }

        assert_eq!(
            wordle_words.len(),
            538,
            "Matt Parker is a better programmer than I 😢"
        );
    }

    #[test]
    fn wordle_answers() {
        let file_contents = fs::read_to_string("res/answers.txt").unwrap();
        let words: Vec<_> = file_contents.split_whitespace().collect();
        let wordle_words = find_words_with_disjoint_character_sets::<5, 5>(words);

        for word_set in wordle_words.iter() {
            println!("{word_set:?}");
        }

        assert_eq!(
            wordle_words.len(),
            0,
            "Matt Parker is a better programmer than I 😢"
        );
    }

    #[test]
    fn is_clique() {
        assert!(CliqueFinder::is_clique(&[WordId::from("abcde")]));
        assert!(CliqueFinder::is_clique(&[
            WordId::from("abcde"),
            WordId::default(),
            WordId::default(),
            WordId::default(),
            WordId::default(),
        ]));
        assert!(CliqueFinder::is_clique(&[
            WordId::from("abcde"),
            WordId::from("fghij"),
            WordId::from("klmno"),
            WordId::from("pqrst"),
            WordId::from("uvwxy"),
        ]));
        assert!(!CliqueFinder::is_clique(&[
            WordId::from("abcde"),
            WordId::from("wxzya"),
        ]),);
    }

    #[test]
    fn word_id() {
        assert_eq!(*WordId::from("aaaaa"), 0b000000_00001);
        assert_eq!(*WordId::from("abcde"), 0b00000_11111);
        assert_eq!(*WordId::from("abcdf"), 0b00001_01111);
    }

    mod single_5_clique {
        use super::*;

        const WORDS: [&'static str; 5] = ["abcde", "fghij", "klmno", "pqrst", "uvwxy"];

        #[test]
        fn word_graph_with_5_clique() {
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(WORDS.to_vec());
            assert_eq!(cliques.len(), 1)
        }
    }
}
