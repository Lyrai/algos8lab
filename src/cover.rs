use crate::misc::*;
use crate::Sets;

#[derive(Clone)]
pub struct Cover {
    pub union: Vec<i32>,
    pub sets: Sets
}

impl Cover {
    pub fn fit_factor(&self) -> usize {
        self.sets.iter().fold(0, |acc, x| acc + x.len()) - self.union.len()
    }

    pub fn set(&mut self, idx: usize, set: Vec<i32>) {
        self.sets[idx] = set;
        self.union = fold_sets(&self.sets);
    }

    pub fn is_cover(&self, n: i32) -> bool {
        is_cover(n, &self.sets)
    }
}