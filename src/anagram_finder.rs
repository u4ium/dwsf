use crate::{id_to_words_map::IdToWordsMap, word_id::WordId};

// TODO: test
// TODO: rename/move
// TODO: needs L=5? (string length)
pub struct AnagramFinder<'a, const N: usize> {
    word_map: IdToWordsMap<'a>,
    result: Vec<[&'a str; N]>,
    current: [&'a str; N],
}

impl<'a, const N: usize> AnagramFinder<'a, N> {
    pub fn new(word_map: IdToWordsMap<'a>) -> Self {
        Self {
            word_map,
            result: Default::default(),
            current: [""; N],
        }
    }

    pub fn find(mut self, cliques: &Vec<[WordId; N]>) -> Vec<[&'a str; N]> {
        for clique in cliques {
            self.find_helper(clique, 0);
        }
        self.result
    }

    fn find_helper(&mut self, clique: &[WordId; N], depth: usize) {
        if depth == N {
            self.result.push(self.current);
            return;
        }

        for w_idx in 0..self.word_map[&clique[depth]].len() {
            self.current[depth] = self.word_map[&clique[depth]][w_idx];
            self.find_helper(clique, depth + 1);
        }
    }
}
