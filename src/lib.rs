use itertools::Itertools;

mod word_id;
use word_id::WordId;
mod id_to_words_map;
use id_to_words_map::IdToWordsMap;
mod clique_finder;
use clique_finder::CliqueFinder;

/// Find sets of N words (each with L distinct characters) that share no characters between them
pub fn find_words_with_disjoint_character_sets<'a, const N: usize, const L: u32>(
    words: Vec<&'a str>,
    //TODO: -> Vec<[&'a str; N]>
) -> Vec<[&'a str; N]> {
    let words: Vec<_> = words
        .into_iter()
        .filter(|&word| WordId::from(word).count_ones() == L)
        .collect();
    let word_map = IdToWordsMap::from_iter(words);
    let word_ids = word_map.keys().cloned().collect();
    let cliques = CliqueFinder::new(word_ids).search();
    construct_result(word_map, cliques)
}

// TODO: test
// TODO: rename/move
// TODO: needs L=5? (string length)
// TODO: don't reallocate Strings
fn construct_result<'a, const N: usize>(
    word_map: IdToWordsMap<'a>,
    cliques: Vec<[WordId; N]>,
) -> Vec<[&'a str; N]> {
    // TODO refactor to avoid filter and produce combinations in smarter way
    // for clique in cliques {
    //     for i in 0..N {
    //         for word in word_map[&clique[i]] {
    //             // add to ret
    //         }
    //     }
    // }

    cliques
        .iter()
        .map(|clique| {
            clique
                .into_iter()
                .flat_map(|cq| &word_map[cq])
                .cloned()
                .combinations(N)
                .filter(is_clique)
                .map(|c| c.try_into().expect("must have {N}"))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn is_clique(word_set: &Vec<&str>) -> bool {
    let mut mask = 0_u32;
    for &word in word_set {
        let word_id = WordId::from(word);

        if mask & *word_id != 0 {
            return false;
        }

        mask |= *word_id;
    }

    true
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::find_words_with_disjoint_character_sets;

    #[test]
    fn wordle_has_831_cliques_of_disjoint_words() {
        let file_contents = fs::read_to_string("res/all_words_5.txt").unwrap();
        let words: Vec<_> = file_contents.split_whitespace().collect();
        let wordle_words = find_words_with_disjoint_character_sets::<5, 5>(words);

        for word_set in wordle_words.iter() {
            println!("{word_set:?}");
        }

        assert_eq!(
            wordle_words.len(),
            831,
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
    fn word_graph_with_5_clique() {
        let words = vec!["abcde", "fghij", "klmno", "pqrst", "uvwxy"];
        let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
        assert_eq!(cliques.len(), 1)
    }
}
