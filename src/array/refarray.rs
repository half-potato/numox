//#![feature(range_contains)]

extern crate num_traits;
extern crate std;

use std::ops::{Index, Range};
//use std::slice::{SliceIndex};

use super::BaseArray;
use super::Iter;

#[derive(Debug)]
pub struct RefArray<'a, T: 'a> {
    data: Vec<&'a T>,
    shape: Vec<usize>,
}

impl<'a, T: num_traits::Num + std::clone::Clone + 'a> BaseArray<'a, T> for RefArray<'a, T> {
    type ArrayType = RefArray<'a, T>;
    type InputData = &'a Vec<T>;

    fn at(&self, ind: &usize) -> &T {
        &self.data[ind.clone()]
    }

    fn get(&self, ind: &Vec<usize>) -> &T {
        let index = self.ele_index(ind);
        assert!(self.size() > index);
        &self.data[index]
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

    /*
    fn iter(&'a self) -> Iter<'a, T, Self> {
        Iter::new(
            self,
            (0..self.size()).collect(),
            Some(self.get_shape().clone()),
        )
    }
    */
}

impl<'a, T: num_traits::Num + std::clone::Clone + 'a> RefArray<'a, T> {
    pub fn new_raw(dat: Vec<&'a T>, shape: Vec<usize>) -> RefArray<'a, T> {
        RefArray {
            data: dat,
            shape: shape.clone(),
        }
    }

    pub fn new(dat: &'a Vec<T>, shape: Vec<usize>) -> RefArray<'a, T> {
        let m_d = dat.iter().collect();
        RefArray {
            data: m_d,
            shape: shape.clone(),
        }
    }
}

impl<'a, T: num_traits::Num + std::clone::Clone> Index<&'a usize> for RefArray<'a, T> {
    type Output = T;
    fn index(&self, index: &usize) -> &T {
        self.get(&self.ele_index_inv(&index))
    }
}

