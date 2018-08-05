extern crate num_traits;
extern crate std;

use super::basearray::BaseArray;

pub struct Iter<'a, T, R> 
    where T : num_traits::Num + std::clone::Clone + 'a, R: BaseArray<'a, T> + 'a
{
    ref_array: &'a R,
    index: usize,
    indices: Vec<usize>,
    out_shape: Option<Vec<usize>>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T, R> Iterator for Iter<'a, T, R> 
    where T : num_traits::Num + std::clone::Clone + 'a, R: BaseArray<'a, T>
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.indices.len() {
            let out = self.ref_array.at(&self.indices[self.index]);
            self.index += 1;
            Some(&out)
        } else {
            None
        }
    }
}

impl<'a, T, R> Iter<'a, T, R> 
    where T : num_traits::Num + std::clone::Clone + 'a, R: BaseArray<'a, T>
{
    pub fn new(
        ref_array: &'a R,
        indices: Vec<usize>,
        out_shape: Option<Vec<usize>>) -> Self
    {
        Iter {
            ref_array: ref_array,
            index: 0,
            indices: indices,
            out_shape: out_shape,
            phantom: std::marker::PhantomData
        }
    }
}
