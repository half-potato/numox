extern crate num_traits;
extern crate std;
use std::ops::{Index, Range};

use super::Iter;
use super::RefArray;

pub trait BaseArray<'a, T: num_traits::Num + std::clone::Clone + 'a> {
    type ArrayType: BaseArray<'a, T>;
    type InputData;

    fn get_shape(&self) -> &Vec<usize>;
    fn at(&self, ind: &usize) -> &T;
    fn get(&self, ind: &Vec<usize>) -> &T;

    fn size(&self) -> usize {
        self.get_shape().iter().product()
    }

    fn ele_index(&self, ind: &Vec<usize>) -> usize {
        let mut index: usize = 0;
        for i in 0..ind.len() {
            let mut mul: usize = 1;
            for j in 0..i {
                mul *= self.get_shape()[j];
            }
            index += mul*ind[i];
        }
        index
    }

    fn get_shape_mut(&mut self) -> &mut Vec<usize>;

    fn ele_index_inv(&self, ind: &usize) -> Vec<usize> {
        let mut index: Vec<usize> = vec![0; self.get_shape().len()];
        let mut ind2 = ind.clone();
        for i in 0..self.get_shape().len() {
            let j = i - self.get_shape().len();
            let place_val: usize = (j..self.get_shape().len()).product();
            index[j] = ind2%place_val;
            ind2 = (ind2/place_val) as usize;
        }
        index
    }

    fn dim(&self) -> usize {
        self.get_shape().len()
    }

    fn squeeze(&mut self) {
        let new_shape = self.get_shape().iter()
            .filter(|x| x != &&(1 as usize))
            .map(|x| *x)
            .collect();
        std::mem::replace(self.get_shape_mut(), new_shape);
    }

    fn slice_iter(&'a self, slice: &usize, dim: usize) -> Iter<'a, T, Self::ArrayType>;
    //fn iter(&self) -> Iter<'a, T, Self::ArrayType>;

    fn slice<R>(&'a self, slices: Vec<R>) -> RefArray<'a, T>
        where R: std::iter::Iterator<Item=usize> + std::clone::Clone
    {
        assert_eq!(self.get_shape().len(), slices.len());
        // Iterate through each slice and get the coordinates for each item
        let mut coords: Vec<Vec<usize>> = slices[0].clone().map(|x| vec![x.clone()]).collect();
        let mut new_shape: Vec<usize> = vec![coords.len()];
        for i in 1..slices.len() {
            let mut new_coords: Vec<Vec<usize>> = Vec::new();
            let mut shape_c = 0;
            for j in slices[i].clone() {
                let mut new_add = coords.clone();
                for k in new_add.iter_mut() {
                    k.push(j);
                }
                new_coords.extend(new_add.iter().cloned());
                shape_c += 1;
            }
            coords = new_coords;
            new_shape.push(shape_c);
        }
        let mut data: Vec<&'a T> = Vec::new();
        for coord in coords.iter() {
            data.push(self.get(coord));
        }
        RefArray::new_raw(data, new_shape)
    }
}

/*
impl<'a, T: num_traits::Num + std::clone::Clone> Index<ArrayIterator<'a, T>> for BaseArray<T> {
    type Output = Array<T>;
    fn index(&self, index: ArrayIterator<'a, T>) -> &BaseArray<T> {
        // Output reference to previous array
    }
}
*/
