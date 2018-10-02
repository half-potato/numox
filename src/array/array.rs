extern crate num_traits;
extern crate std;
extern crate rand;

use std::ops::{Index, Range};

use rand::prelude::*;
use rand::distributions::{Standard, Distribution, Uniform, StandardNormal, uniform};
use super::BaseArray;
use super::MDArray;
use super::RefArrayMut;
use super::MutArray;
use super::Iter;
use super::RefArray;

#[derive(Clone, Debug)]
pub struct Array<T> {
    data: Vec<T>,
    shape: Vec<usize>,
}


impl<'a, T> Array<T>
    where T: num_traits::Num + std::clone::Clone + 'a, 
             Standard: Distribution<T>,
             StandardNormal: Distribution<T>,
{

    pub fn rand_gaussian(shape: &Vec<usize>, min: &T, max: &T) -> Array<T> {
        Self::rand_range(shape, min, max, StandardNormal)
    }

    pub fn rand_uniform(shape: &Vec<usize>, min: &T, max: &T) -> Array<T> {
        Self::rand_range(shape, min, max, Standard)
    }
    
    pub fn rand_range<R> (shape: &Vec<usize>, min: &T, max: &T, distrib: R) -> Array<T> 
        where R: rand::distributions::Distribution<T>
    {
        let mut shell = Self::new_full(shape, num_traits::identities::one());
        let mut rng = thread_rng();
        let range = max.clone()-min.clone();
        for i in 0..shell.size() {
            let val: T = rng.sample(&distrib);
            shell.set_at(&i, &(range.clone()*val+min.clone()));
        }
        shell
    }
    
    pub fn rand_distrib<R> (shape: &Vec<usize>, mul: &Option<T>, distrib: R) -> Array<T> 
        where R: rand::distributions::Distribution<T>
    {
        let mut shell = Self::new_full(shape, num_traits::identities::one());
        let mut rng = thread_rng();
        for i in 0..shell.size() {
            let val: T = rng.sample(&distrib);
            shell.set_at(&i, &(match mul.clone() {
                Some(v) => (val * v),
                None => val,
            }));
            
        }
        shell
    }

}

impl<'a, T> Array<T>
    where T: num_traits::Num + std::clone::Clone + 'a
{

    pub fn new(dat: Vec<T>, shape: Vec<usize>) -> Array<T> {
        Array {
            data: dat,
            shape: shape,
        }
    }

    pub fn new_full(shape: &Vec<usize>, v: T) -> Array<T> {
        let mut dat_size = shape[0];
        for i in 1..shape.len() {
            dat_size *= shape[i];
        }
        let dat = vec![v; dat_size];
        Self::new(dat, shape.clone())
    }

    pub fn zeros(shape: &Vec<usize>) -> Array<T> {
        Self::new_full(shape, num_traits::identities::zero())
    }

    pub fn zeros_like(other: &Array<T>) -> Array<T> {
        Self::zeros(other.get_shape())
    }

    pub fn ones(shape: &Vec<usize>) -> Array<T> {
        Self::new_full(shape, num_traits::identities::one())
    }

    pub fn ones_like(other: &Array<T>) -> Array<T> {
        Self::ones(&other.get_shape())
    }

    pub fn eye(size: usize) -> Array<T> {
        let mut arr = Self::zeros(&vec![size, size]);
        for i in 0..size {
            *arr.get_mut(&vec![i,i]) = num_traits::identities::one();
        }
        arr
    }

    pub fn reference(&'a self) -> RefArray<'a, T> {
        RefArray::new(&self.data, self.shape.clone())
    }

    pub fn slice_mut<R>(&'a mut self, slices: &Vec<R>) -> RefArrayMut<'a, T> 
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
        RefArrayMut::new_raw(self, new_shape, coords)
    }
}


impl<'a, T> BaseArray<'a, T> for Array<T>
    where T: num_traits::Num + std::clone::Clone + 'a
{
    type ArrayType = Array<T>;
    type InputData = Vec<T>;

    fn at(&self, ind: &usize) -> &T {
        &self.data[ind.clone()]
    }

    fn get(&self, ind: &Vec<usize>) -> &T {
        let index = self.ele_index(ind);
        assert!(self.size() > index);
        self.at(&index)
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

impl<'a, T> MutArray<'a, T> for Array<T>
    where T: num_traits::Num + std::clone::Clone + 'a
{
    fn at_mut(&mut self, ind: &usize) -> &mut T {
        &mut (self.data[ind.clone()])
    }

    fn get_mut(&mut self, ind: &Vec<usize>) -> &mut T {
        let index = self.ele_index(ind);
        assert!(self.size() > index);
        self.at_mut(&index)
    }

    fn set_at(&mut self, ind: &usize, val: &T) {
        *self.at_mut(ind) = val.clone();
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

impl<'a, T> MDArray<'a, T> for Array<T> 
    where T: num_traits::Num + std::clone::Clone + 'a
{
}

impl<'a, T: num_traits::Num + std::clone::Clone> Index<&'a usize> for Array<T> {
    type Output = T;
    fn index(&self, index: &usize) -> &T {
        self.get(&self.ele_index_inv(&index))
    }
}
