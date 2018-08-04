#![feature(range_contains)]

extern crate num_traits;
extern crate std;
use std::ops::{IndexMut, Mul, Range};
//use std::slice::{SliceIndex};

pub struct Array<T> {
    data: Vec<T>,
    shape: Vec<usize>,
}

impl<T: num_traits::Num + std::clone::Clone> Array<T> {

    pub fn new_full(shape: &Vec<usize>, v: T) -> Array<T> {
        let mut dat_size = shape[0];
        for i in 1..shape.len() {
            dat_size *= shape[i];
        }
        let dat = vec![v; dat_size];
        Array {
            data: dat,
            shape: shape.clone(),
        }
    }

    pub fn zeros(shape: &Vec<usize>) -> Array<T> {
        Array::new_full(shape, num_traits::identities::zero())
    }

    pub fn zeros_like(other: &Array<T>) -> Array<T> {
        Array::zeros(&other.shape)
    }

    pub fn ones(shape: &Vec<usize>) -> Array<T> {
        Array::new_full(shape, num_traits::identities::one())
    }

    pub fn ones_like(other: &Array<T>) -> Array<T> {
        Array::ones(&other.shape)
    }

    pub fn eye(size: usize) -> Array<T> {
        let mut arr = Array::zeros(&vec![size, size]);
        for i in 0..size {
            *arr.get_mut(&vec![i,i]) = num_traits::identities::one();
        }
        arr
    }

    pub fn ele_index(&self, ind: &Vec<usize>) -> usize {
        let mut index: usize = 0;
        for i in 0..ind.len() {
            let mut mul: usize = 1;
            for j in 0..i {
                mul *= self.shape[j];
            }
            index += mul*ind[i];
        }
        index
    }

    pub fn ele_index_inv(&self, ind: &usize) -> Vec<usize> {
        let mut index: Vec<usize> = vec![0; self.shape.len()];
        let mut ind2 = ind.clone();
        for i in 0..self.shape.len() {
            let j = i - self.shape.len();
            let place_val: usize = (j..self.shape.len()).product();
            index[j] = ind2%place_val;
            ind2 = (ind2/place_val) as usize;
        }
        index
    }

    pub fn dim(&self) -> usize {
        self.shape.len()
    }

    pub fn squeeze_dim(&self) -> usize {
        let u: Vec<&usize> = self.shape.iter().filter(|x| x != &&(1 as usize)).collect();
        u.len()
    }

    pub fn squeeze(&mut self) {
        self.shape = self.shape.iter()
            .filter(|x| x != &&(1 as usize))
            .map(|x| *x)
            .collect();
    }

    pub fn get(&self, ind: &Vec<usize>) -> &T {
        let index = self.ele_index(ind);
        assert!(self.data.len() > index);
        &self.data[index]
    }

    pub fn get_mut(&mut self, ind: &Vec<usize>) -> &mut T {
        let index = self.ele_index(ind);
        assert!(self.data.len() > index);
        &mut self.data[index]
    }

    pub fn slice_iter(&self, slice: &usize, dim: usize) -> ArrayIterator<T> {
        let new_shape: Vec<usize> = [&self.shape[0..dim], &self.shape[dim..self.shape.len()]].concat();
        let indices = (0..self.data.len())
            .filter(|i| {
                slice == &self.ele_index_inv(&i)[dim]
            })
            .collect();
        ArrayIterator{
            indices: indices,
            index: 0,
            ref_array: self,
            out_shape: Some(new_shape),
        }
    }

    pub fn iter(&self) -> ArrayIterator<T> {
        ArrayIterator{
            indices: (0..self.data.len()).collect(),
            index: 0,
            ref_array: self,
            out_shape: Some(self.shape.clone()),
        }
    }

    /*
    fn slice_range_iter(&self, slice: &Range<usize>, dim: usize) -> ArrayIterator<T> {
        let indices = (0..self.data.len())
            .filter(|i| {
                slice.contains(&self.ele_index_inv(&i)[dim])
            })
            .collect();
        ArrayIterator{
            indices: indices,
            index: 0,
            ref_array: self,
        }
    }
    */

    pub fn product(&self, other: &Array<T>) -> Array<T> {
        assert_eq!(self.shape, other.shape);
        let mut out = Array::zeros_like(self);
        for i in 0..self.data.len() {
            out.data[i] = self.data[i].clone() * other.data[i].clone();
        }
        out
    }

    pub fn scalar_product(&self, other: &T) -> Array<T> {
        let mut out = Array::zeros_like(self);
        for i in 0..self.data.len() {
            out.data[i] = self.data[i].clone() * other.clone();
        }
        out
    }

    pub fn add(&self, other: &Array<T>) -> Array<T> {
        assert_eq!(self.shape, other.shape);
        let mut out = Array::zeros_like(self);
        for i in 0..self.data.len() {
            out.data[i] = self.data[i].clone() + other.data[i].clone();
        }
        out
    }

    pub fn scalar_add(&self, other: &T) -> Array<T> {
        let mut out = Array::zeros_like(self);
        for i in 0..self.data.len() {
            out.data[i] = self.data[i].clone() + other.clone();
        }
        out
    }

    pub fn inner_product(&self, other: &Array<T>) -> Array<T> {
        assert_eq!(self.shape.last().unwrap(), &other.shape[0]);
        // Iterate over the first dim 
        let a = &self.shape[0..self.shape.len()-1];
        let b = &other.shape[1..];
        let new_shape: Vec<usize> = [a, b].concat();

        let mut out = Array::zeros(&new_shape);
        out
    }
}

pub struct ArrayIterator<'a, T: num_traits::Num + std::clone::Clone + 'a> {
    ref_array: &'a Array<T>,
    index: usize,
    indices: Vec<usize>,
    out_shape: Option<Vec<usize>>,
}

impl<'a, T: num_traits::Num + std::clone::Clone + 'a> ArrayIterator<'a, T> {
    pub fn collect(&self) -> &Array<T> {
    }
}

impl<'a, T: num_traits::Num + std::clone::Clone + 'a> Iterator for ArrayIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.indices.len() {
            let out = self.ref_array.get(
                &self.ref_array.ele_index_inv(&self.indices[self.index]));
            self.index += 1;
            Some(out)
        } else {
            None
        }
    }
}

// Impl dot product
impl<T: num_traits::Num + std::clone::Clone> Mul<Array<T>> for Array<T> {
    type Output = Array<T>;
    fn mul(self, other: Array<T>) -> Array<T> {
        // Implement dot product
        let mut out: Array<T> = Array::zeros(&vec![self.shape[0], other.shape[1]]);
        out
    }
}

/*
impl<T> IndexMut<[usize]> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
    }
}
*/
