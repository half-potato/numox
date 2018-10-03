extern crate std;
extern crate rand;
extern crate num;

use std::ops;
use std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct Array<T>
    where T: std::clone::Clone
{
    pub data: Vec<T>,
    pub shape: Vec<usize>,
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

    // Convert multidimensional index to index in 1d data
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

    // Convert index in 1d data to multidimensional index 
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

    pub fn squeeze(&mut self) {
        let new_shape = self.shape.iter()
            .filter(|x| x != &&(1 as usize))
            .map(|x| *x)
            .collect();
        std::mem::replace(&mut self.shape, new_shape);
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

