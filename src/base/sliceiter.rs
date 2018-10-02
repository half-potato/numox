extern crate std;

use std::iter::Iterator;

//macro_rules!

#[derive(Clone, Debug)]
pub struct Iter {
    indices: Vec<usize>,
    i: usize,
}

impl Iterator for Iter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        if self.i >= self.indices.len() {
            None
        } else {
            Some(self.indices[self.i])
        }
    }
}

#[derive(Clone, Debug)]
pub enum IndexIter {
    Index(usize),
    Range(std::ops::Range<usize>),
    Slice(Iter),
    Whole(i8), // Negative numbers take the whole slice
}
