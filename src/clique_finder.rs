use crate::word_id::WordId;

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

    pub fn search(mut self) -> Vec<[WordId; N]> {
        self.search_helper(0, 0);
        self.result
    }

    fn search_helper(&mut self, start_index: usize, depth: usize) {
        // collect the current clique into a single bitset for fast intersection tests
        let rep = self
            .current
            .iter()
            .take(depth)
            .fold(0_u32, |a, &word_id| (a | *word_id));

        for j in start_index..=self.word_ids.len() - (N - depth) {
            if rep & *self.word_ids[j] == 0 {
                // this word can be added to the bitset without letter intersections
                self.current[depth] = self.word_ids[j];
                if depth + 1 == N {
                    // current is a complete N-clique, so add it to the result buffer
                    self.result.push(self.current.clone());
                } else {
                    // current is a k-clique, where k<N,
                    // so recurse to find all possible (k+1)th words
                    self.search_helper(j + 1, depth + 1);
                }
            }
        }
    }
}
