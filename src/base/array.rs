extern crate std;
extern crate rand;
extern crate num_traits;

use rand::prelude::*;
use rand::distributions::{Standard, Distribution, StandardNormal};
use std::ops;
use self::num_traits::identities::{zero};
use super::sliceiter::IndexIter;
use std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct Array<T>
    where T: std::clone::Clone
{
    data: Vec<T>,
    shape: Vec<usize>,
}

impl<T> Array<T> 
    where T: std::clone::Clone
{

    pub fn new(shape: Vec<usize>, def_val: T) -> Array<T> {
        Array {
            data: vec![def_val; shape.iter().product()],
            shape: shape,
        }
    }

    pub fn len(&self) -> usize {
        self.shape.iter().product()
    }

    pub fn ele_index(&self, ind: Vec<usize>) -> usize {
        assert_eq!(ind.len(), self.shape.len(), "Tried to index array using wrong number of dimensions");
        let mut index: usize = 0;
        for i in 0..ind.len() {
            assert!(ind[i] < self.shape[i], "Index exceeds dimension of array");
            let axis_val: usize = (0..i).map(|x| self.shape[x]).product();
            index += ind[i]*axis_val;
        }
        index
    }

    pub fn ele_index_inv(&self, ind: usize) -> Vec<usize> {
        assert!(ind < self.len());
        let mut index: Vec<usize> = vec![0; self.shape.len()];
        let mut ind2 = ind.clone();
        for i in 0..self.shape.len() {
            if ind2 == 0{
                break;
            }
            // Index in reverse
            let j = self.shape.len() - i - 1;
            // 
            let place_val: usize =
                (0..j)
                .map(|x| self.shape[x])
                .product();
            index[j] = ((ind2 as f64)/(place_val as f64)).floor() as usize;
            ind2 = ind2%place_val;
        }
        index
    }

    pub fn get(&self, index: Vec<usize>) -> &T {
        assert_eq!(index.len(), self.shape.len());
        &self.data[self.ele_index(index)]
    }

    pub fn get_mut(&mut self, index: Vec<usize>) -> &mut T {
        let ind = self.ele_index(index);
        &mut self.data[ind]
    }

    pub fn subset(&self, index: Vec<IndexIter>) -> Array<T> {
        self.slice(self.make_slice(index))
    }

    pub fn set_subset(&mut self, index: Vec<IndexIter>, val: Array<T>) {
        let (coords, new_shape) = self.slice_to_ind(self.make_slice(index));
        assert_eq!(new_shape, val.shape);
        for (i, coord) in coords.iter().enumerate() {
            self[coord.clone()] = val.data[i].clone();
        }
    }

    pub fn squeeze(&mut self) {
        let new_shape = self.shape.iter()
            .filter(|x| x != &&(1 as usize))
            .map(|x| *x)
            .collect();
        std::mem::replace(&mut self.shape, new_shape);
    }
    
    pub fn make_slice(&self, index: Vec<IndexIter>) -> Vec<Vec<usize>> {
        let mut slicer: Vec<Vec<usize>> = Vec::new();
        for i in 0..self.shape.len() {
            if i >= index.len() {
                slicer.push((0..self.shape[i]).collect());
            } else {
                slicer.push((match index[i].clone() {
                    IndexIter::Index(ind) => (ind..ind+1).collect(),
                    IndexIter::Range(ref rng) => (rng.clone()).collect(),
                    IndexIter::Whole(_) => (0..self.shape[i]).collect(),
                    IndexIter::Slice(ind) => (0..self.shape[i]).collect(),
                }));
            }
        }
        slicer
    }

    pub fn slice_to_ind(&self, slices: Vec<Vec<usize>> ) -> (Vec<Vec<usize>>, Vec<usize>)
    {
        assert_eq!(self.shape.len(), slices.len());
        // Iterate through each slice and get the coordinates for each item
        let mut coords: Vec<Vec<usize>> = slices[0]
            .clone()
            .iter()
            .map(|x| vec![x.clone()])
            .collect();
        let mut new_shape: Vec<usize> = vec![coords.len()];
        for i in 1..slices.len() {
            let mut new_coords: Vec<Vec<usize>> = Vec::new();
            let mut shape_c = 0;
            for j in slices[i].iter() {
                let mut new_add = coords.clone();
                for k in new_add.iter_mut() {
                    k.push(j.clone());
                }
                new_coords.extend(new_add.iter().cloned());
                shape_c += 1;
            }
            coords = new_coords;
            new_shape.push(shape_c);
        }
        (coords, new_shape)
    }

    pub fn slice(&self, slices: Vec<Vec<usize>>) -> Array<T>
    {
        let (coords, new_shape) = self.slice_to_ind(slices);
        let mut data: Vec<T> = Vec::new();
        for coord in coords.iter() {
            data.push(self[coord.clone()].clone());
        }
        Array {
            data: data,
            shape: new_shape,
        }
    }

}

impl<'a, T> ops::Index<Vec<usize>> for Array<T>
    where T: std::clone::Clone
{
    type Output = T;
    fn index(&self, index: Vec<usize>) -> &Self::Output {
        self.get(index)
    }
}

impl<'a, T> ops::IndexMut<Vec<usize>> for Array<T>
    where T: std::clone::Clone
{
    fn index_mut(&mut self, index: Vec<usize>) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<'a, T> Array<T>
    where T: num_traits::Num + std::clone::Clone + 'a, 
             Standard: Distribution<T>,
             StandardNormal: Distribution<T>,
{

    pub fn rand_gaussian(shape: Vec<usize>, min: T, max: T) -> Array<T> {
        Self::rand_range(shape, min, max, StandardNormal)
    }

    pub fn rand_uniform(shape: Vec<usize>, min: T, max: T) -> Array<T> {
        Self::rand_range(shape, min, max, Standard)
    }
    
    pub fn rand_range<R> (shape: Vec<usize>, min: T, max: T, distrib: R) -> Array<T> 
        where R: rand::distributions::Distribution<T>
    {
        let mut shell = Self::new(shape, num_traits::identities::one());
        let mut rng = thread_rng();
        let range = max.clone()-min.clone();
        for i in 0..shell.len() {
            let val: T = rng.sample(&distrib);
            shell.data[i] = range.clone()*val+min.clone();
        }
        shell
    }
    
    pub fn rand_distrib<R> (shape: Vec<usize>, mul: Option<T>, distrib: R) -> Array<T> 
        where R: rand::distributions::Distribution<T>
    {
        let mut shell = Self::new(shape, num_traits::identities::one());
        let mut rng = thread_rng();
        for i in 0..shell.len() {
            let val: T = rng.sample(&distrib);
            shell.data[i] = match mul.clone() {
                Some(v) => (val * v),
                None => val,
            };
            
        }
        shell
    }

}

impl<'a, T> Array<T>
    where T: num_traits::Num + std::clone::Clone + 'a
{
    pub fn zeros(shape: Vec<usize>) -> Array<T> {
        Self::new(shape, num_traits::identities::zero())
    }

    pub fn zeros_like(other: &Array<T>) -> Array<T> {
        Self::zeros(other.shape.clone())
    }

    pub fn ones(shape: Vec<usize>) -> Array<T> {
        Self::new(shape, num_traits::identities::one())
    }

    pub fn ones_like(other: &Array<T>) -> Array<T> {
        Self::ones(other.shape.clone())
    }

    pub fn eye(len: usize) -> Array<T> {
        let mut arr = Self::zeros(vec![len, len]);
        for i in 0..len {
            arr[vec![i,i]] = num_traits::identities::one();
        }
        arr
    }

    pub fn product(&self, other: &Array<T>) -> Array<T> {
        assert_eq!(self.shape, other.shape);
        let mut data = vec![zero(); self.len()];
        for i in 0..self.len() {
            data[i] = self.data[i].clone() * other.data[i].clone();
        }
        Array {
            shape: self.shape.clone(),
            data: data
        }
    }

    pub fn scalar_product(&self, other: &T) -> Array<T> {
        let mut data = vec![zero(); self.len()];
        for i in 0..self.len() {
            data[i] = self.data[i].clone() * other.clone();
        }
        Array {
            shape: self.shape.clone(),
            data: data
        }
    }

    pub fn add(&self, other: &Array<T>) -> Array<T> {
        assert_eq!(self.shape, other.shape);
        let mut data = vec![num_traits::identities::zero(); self.len()];
        for i in 0..self.len() {
            data[i] = self.data[i].clone() + other.data[i].clone();
        }
        Array {
            shape: self.shape.clone(),
            data: data
        }
    }

    pub fn scalar_add(&self, other: &T) -> Array<T> {
        let mut data = vec![num_traits::identities::zero(); self.len()];
        for i in 0..self.len() {
            data[i] = self.data[i].clone() + other.clone();
        }
        Array {
            shape: self.shape.clone(),
            data: data
        }
    }

    pub fn inner_product(&self, other: &Array<T>) -> Array<T> {
        // Iterate over the first dim 
        let s1 = self.shape.clone();
        let s2 = other.shape.clone();
        assert!(s1.len() > 1);
        assert!(s2.len() > 1);
        assert_eq!(s1.last().unwrap(), &s2[0]);
        let a = &s1[0..s1.len()-1];
        let b = &s2[1..];
        let new_shape: Vec<usize> = [a, b].concat();

        let mut out: Array<T> = Array::zeros(new_shape);
        for i in 0..self.len() {
            let c1 = self.ele_index_inv(i);
            if c1[s1.len()-1] != 0 {
                continue;
            }
            for j in 0..other.len() {
                let c2 = other.ele_index_inv(j);
                if c2[0] != 0 {
                    continue;
                }
                // Iterate over everything except the inner 2 dims
                let pos: Vec<usize> = [&c1[0..c1.len()-1], &c2[1..]].concat();
                // Now dot the 2 inner dims as vectors
                // TODO: Use the slices instead by writing a function to create slices
                let mut val: T = num_traits::identities::zero();
                for k in 0..s2[0] {
                    let mut rhs = c1.clone();
                    rhs[s1.len()-1] = k;
                    let mut lhs = c2.clone();
                    lhs[0] = k;
                    val = val + self.get(rhs).clone() * other.get(lhs).clone();
                }
                out.data[self.ele_index(pos)] =  val;
            }
        }
        out
    }
}
