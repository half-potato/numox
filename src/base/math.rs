extern crate std;
extern crate num;

use super::array::Array;
use self::num::traits::identities::{zero, one};
use self::num::traits;

impl<'a, T> Array<T>
    where T: traits::Num + std::clone::Clone + 'a
{
    pub fn zeros(shape: Vec<usize>) -> Array<T> {
        Self::new(shape, traits::identities::zero())
    }

    pub fn zeros_like(other: &Array<T>) -> Array<T> {
        Self::zeros(other.shape.clone())
    }

    pub fn ones(shape: Vec<usize>) -> Array<T> {
        Self::new(shape, one())
    }

    pub fn ones_like(other: &Array<T>) -> Array<T> {
        Self::ones(other.shape.clone())
    }

    pub fn eye(len: usize) -> Array<T> {
        let mut arr = Self::zeros(vec![len, len]);
        for i in 0..len {
            arr[vec![i,i]] = one();
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
        let mut data = vec![traits::identities::zero(); self.len()];
        for i in 0..self.len() {
            data[i] = self.data[i].clone() + other.data[i].clone();
        }
        Array {
            shape: self.shape.clone(),
            data: data
        }
    }

    pub fn scalar_add(&self, other: &T) -> Array<T> {
        let mut data = vec![traits::identities::zero(); self.len()];
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
                let mut val: T = traits::identities::zero();
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
