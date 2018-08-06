extern crate num_traits;
extern crate std;
use super::BaseArray;
use super::Array;
use super::MutArray;

use std::ops::{Mul};

pub trait MDArray<'a, T> : BaseArray<'a, T>
    where T: num_traits::Num + std::clone::Clone + 'a
{

    fn product(&self, other: &Self::ArrayType) -> Array<T> {
        assert_eq!(self.get_shape(), other.get_shape());
        let mut data = vec![num_traits::identities::zero(); self.size()];
        for i in 0..self.size() {
            data[i] = self.at(&i).clone() * other.at(&i).clone();
        }
        Array::new(data, self.get_shape().clone())
    }

    fn scalar_product(&self, other: &T) -> Array<T> {
        let mut data = vec![num_traits::identities::zero(); self.size()];
        for i in 0..self.size() {
            data[i] = self.at(&i).clone() * other.clone();
        }
        Array::new(data, self.get_shape().clone())
    }

    fn add(&self, other: &Self::ArrayType) -> Array<T> {
        assert_eq!(self.get_shape(), other.get_shape());
        let mut data = vec![num_traits::identities::zero(); self.size()];
        for i in 0..self.size() {
            data[i] = self.at(&i).clone() + other.at(&i).clone();
        }
        Array::new(data, self.get_shape().clone())
    }

    fn scalar_add(&self, other: &T) -> Array<T> {
        let mut data = vec![num_traits::identities::zero(); self.size()];
        for i in 0..self.size() {
            data[i] = self.at(&i).clone() + other.clone();
        }
        Array::new(data, self.get_shape().clone())
    }

    fn inner_product(&self, other: &Self::ArrayType) -> Array<T> {
        // Iterate over the first dim 
        let s1 = self.get_shape();
        let s2 = other.get_shape();
        assert!(s1.len() > 1);
        assert!(s2.len() > 1);
        assert_eq!(s1.last().unwrap(), &s2[0]);
        let a = &s1[0..s1.len()-1];
        let b = &s2[1..];
        let new_shape: Vec<usize> = [a, b].concat();

        let mut out = Array::zeros(&new_shape);
        for i in 0..self.size() {
            let c1 = self.ele_index_inv(&i);
            if c1[s1.len()-1] != 0 {
                continue;
            }
            for j in 0..other.size() {
                let c2 = other.ele_index_inv(&j);
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
                    val = val + self.get(&rhs).clone() * other.get(&lhs).clone();
                }
                out.set_at(&self.ele_index(&pos), &val);
            }
        }
        out
    }
}
