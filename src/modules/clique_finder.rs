use crate::modules::word_id::WordId;

pub struct CliqueFinder<const N: usize> {
    word_ids: Vec<WordId>,
    result: Vec<[WordId; N]>,
    current: [WordId; N],
}

impl<const N: usize> CliqueFinder<N> {
    #[inline]
    pub fn new(word_ids: Vec<WordId>) -> Self {
        Self {
            word_ids,
            result: Default::default(),
            current: [Default::default(); N],
        }
    }

    #[inline]
    pub fn search(mut self) -> Vec<[WordId; N]> {
        if N > self.word_ids.len() {
            return self.result;
        }
        self.search_helper(0, 0, 0);
        self.result
    }

    fn search_helper(&mut self, start_index: usize, depth: usize, rep: u32) {
        if depth == N {
            // current is a complete N-clique, so add it to the result buffer
            self.result.push(self.current.clone());
        } else {
            // current is a (k-1)-clique, where k<N,
            // so find all possible k-cliques containing current
            for j in start_index..=self.word_ids.len() - (N - depth) {
                let word_id = self.word_ids[j];
                if (rep & *word_id) == 0 {
                    // this word can be added to the bitset without letter intersections
                    self.current[depth] = word_id;
                    self.search_helper(j + 1, depth + 1, rep | *word_id);
                }
            }
        }
    }
}
