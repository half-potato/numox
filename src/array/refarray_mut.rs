//#![feature(range_contains)]

extern crate num_traits;
extern crate std;

use std::ops::{Index};

use super::BaseArray;
use super::MDArray;
use super::MutArray;
use super::Array;
use super::Iter;

#[derive(Debug)]
pub struct RefArrayMut<'a, T: 'a> {
    ref_array: &'a mut Array<T>,
    shape: Vec<usize>,
    index: Vec<Vec<usize>>,
}

impl<'a, T> RefArrayMut<'a, T>
    where T: num_traits::Num + std::clone::Clone + 'a
{
    pub fn new_raw(ref_array: &'a mut Array<T>, shape: Vec<usize>, index: Vec<Vec<usize>>) -> RefArrayMut<'a, T> {
        RefArrayMut {
            ref_array: ref_array,
            shape: shape.clone(),
            index: index,
        }
    }
}

impl<'a, T> BaseArray<'a, T> for RefArrayMut<'a, T> 
    where T: num_traits::Num + std::clone::Clone + 'a
{
    type ArrayType = RefArrayMut<'a, T>;
    type InputData = &'a mut Vec<T>;

    fn at(&self, ind: &usize) -> &T {
        let index = self.ref_array.ele_index(&self.index[ind.clone()]);
        &self.ref_array.at(&index)
    }

    fn get(&self, ind: &Vec<usize>) -> &T {
        let index = self.ele_index(ind);
        assert!(self.size() > index);
        &self.at(&index)
    }

    fn get_shape(&self) -> &Vec<usize> {
        &self.shape
    }

    fn get_shape_mut(&mut self) -> &mut Vec<usize> {
        &mut self.shape
    }

    fn slice_iter(&'a self, slice: &usize, dim: usize) -> Iter<'a, T, Self> {
        let new_shape = [&self.get_shape()[0..dim], &self.get_shape()[dim..self.get_shape().len()]].concat();
        let indices = (0..self.size())
            .filter(|i| {
                slice == &self.ele_index_inv(&i)[dim]
            })
            .collect();
        Iter::new (
            self,
            indices,
            Some(new_shape),
        )
    }

    fn iter(&'a self) -> Iter<'a, T, Self> {
        Iter::new(
            self,
            (0..self.size()).collect(),
            Some(self.get_shape().clone()),
        )
    }
}

impl<'a, T> MutArray<'a, T> for RefArrayMut<'a, T>
    where T: num_traits::Num + std::clone::Clone + 'a
{

    fn at_mut(&mut self, ind: &usize) -> &mut T {
        let index = self.ref_array.ele_index(&self.index[ind.clone()]);
        self.ref_array.at_mut(&index)
    }

    fn get_mut(&mut self, ind: &Vec<usize>) -> &mut T {
        let index = self.ele_index(ind);
        assert!(self.size() > index);
        self.at_mut(&index)
    }

    fn set_at(&mut self, ind: &usize, val: &T) {
        std::mem::replace(self.at_mut(ind), val.clone());
    }

    fn set<R: BaseArray<'a, T>>(&mut self, other: R) {
        assert_eq!(self.get_shape(), other.get_shape());
        for i in 0..self.size() {
            self.set_at(&i, other.at(&i));
        }
    }

    fn set_scalar(&mut self, other: T) {
        for i in 0..self.size() {
            self.set_at(&i, &other);
        }
    }
}

impl<'a, T> MDArray<'a, T> for RefArrayMut<'a, T> 
    where T: num_traits::Num + std::clone::Clone + 'a
{
}

impl<'a, T: num_traits::Num + std::clone::Clone> Index<&'a usize> for RefArrayMut<'a, T> {
    type Output = T;
    fn index(&self, index: &usize) -> &T {
        self.get(&self.ele_index_inv(&index))
    }
}
