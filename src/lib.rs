mod modules;
use modules::*;

/// Find sets of N words (each with L distinct characters) that share no characters between them
pub fn find_words_with_disjoint_character_sets<'a, const N: usize, const L: u32>(
    words: Vec<&'a str>,
) -> Vec<[&'a str; N]> {
    // TODO: move into IdToWordsMap Constructor, generalize over IsFixedLength (L or None)
    let words: Vec<_> = words
        .into_iter()
        .filter(|&word| WordId::from(word).count_ones() == L)
        .collect();
    let word_map = IdToWordsMap::from_iter(words);
    //TODO: avoid collect with Impl Iter
    let word_ids = word_map.keys().cloned().collect();
    let cliques = CliqueFinder::new(word_ids).search();
    AnagramFinder::new(word_map).find(&cliques)
}

#[cfg(test)]
mod tests {
    use crate::find_words_with_disjoint_character_sets;
    use std::fs;

    #[test]
    #[ignore = "slow"]
    fn wordle_has_831_cliques_of_disjoint_words() {
        let file_contents = fs::read_to_string("res/all_words.txt").unwrap();
        let words: Vec<_> = file_contents
            .split_whitespace()
            .filter(|word| word.len() == 5)
            .collect();
        let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);

        assert_eq!(
            cliques.len(),
            831,
            "Matt Parker is a better programmer than I 😢"
        );
    }

    #[test]
    fn word_graph_with_5_clique() {
        let words = vec!["abcde", "fghij", "klmno", "pqrst", "uvwxy"];
        let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
        assert_eq!(cliques.len(), 1)
    }

    #[test]
    fn word_graph_with_5_clique_and_anagram() {
        {
            let words = vec!["abcde", "fghij", "klmno", "pqrst", "uvwxy", "vuwxy"];
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
            assert_eq!(cliques.len(), 2)
        }

        {
            let words = vec!["abcde", "fghij", "gfhij", "klmno", "pqrst", "uvwxy"];
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
            assert_eq!(cliques.len(), 2)
        }
    }

    #[test]
    fn word_graph_with_two_5_cliques() {
        {
            let words = vec!["abcde", "fghij", "klmno", "pqrst", "uvwxy", "uvwxz"];
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
            assert_eq!(cliques.len(), 2)
        }
        {
            let words = vec!["abcde", "abcdz", "fghij", "klmno", "pqrst", "uvwxy"];
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
            assert_eq!(cliques.len(), 2)
        }
    }

    #[test]
    fn word_graph_with_two_5_cliques_and_one_anagram_for_one() {
        {
            let words = vec![
                "abcde", "fghij", "klmno", "pqrst", "uvwxy", "uvwxz", "uvwxz",
            ];
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
            assert_eq!(cliques.len(), 3)
        }
        {
            let words = vec![
                "abcde", "abcdz", "bacdz", "fghij", "klmno", "pqrst", "uvwxy",
            ];
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
            assert_eq!(cliques.len(), 3)
        }
    }

    #[test]
    fn word_graph_with_two_5_cliques_and_one_anagram_for_each() {
        {
            let words = vec![
                "abcde", "fghij", "klmno", "pqrst", "uvwxy", "uvwxz", "uvwxz", "vuwxz",
            ];
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
            assert_eq!(cliques.len(), 4)
        }
        {
            let words = vec![
                "abcde", "bacde", "abcdz", "bacdz", "fghij", "klmno", "pqrst", "uvwxy",
            ];
            let cliques = find_words_with_disjoint_character_sets::<5, 5>(words);
            assert_eq!(cliques.len(), 4)
        }
    }
}
