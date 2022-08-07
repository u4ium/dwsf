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
) -> Vec<[String; N]> {
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
// TODO: rename/move
// TODO: needs L=5? (string length)
fn construct_result<const N: usize>(
    word_map: IdToWordsMap,
    cliques: Vec<[WordId; N]>,
) -> Vec<[String; N]> {
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
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::find_words_with_disjoint_character_sets;

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
            15,
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
