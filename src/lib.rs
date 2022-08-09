mod word_id;
use word_id::WordId;
mod id_to_words_map;
use id_to_words_map::IdToWordsMap;
mod clique_finder;
use clique_finder::CliqueFinder;
mod anagram_finder;
use anagram_finder::AnagramFinder;

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
    AnagramFinder::new(word_map).find(&cliques)
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
