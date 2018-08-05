extern crate num_traits;
extern crate std;
use super::iterarray::IterArray;
use super::basearray::BaseArray;

pub trait MDArray<'a, T: num_traits::Num + std::clone::Clone + 'a + std::fmt::Debug> : BaseArray<T> + IterArray<'a, T> {
    fn product<R: MDArray<'a, T>>(&self, other: &R) -> Self::ArrayType {
        assert_eq!(self.get_shape(), other.get_shape());
        let mut out = R::zeros_like(other);
        for i in 0..self.get_data().len() {
            out.get_data()[i] = self.get_data()[i].clone() * other.get_data()[i].clone();
        }
        //out
    }

    fn scalar_product(&self, other: &T) -> Self::ArrayType {
        let mut out: Self::ArrayType = BaseArray::zeros_like(self);
        for i in 0..self.get_data().len() {
            out.get_data_mut()[i] = self.get_data()[i].clone() * other.clone();
        }
        out
    }

    fn add(&self, other: &Self::ArrayType) -> Self::ArrayType {
        assert_eq!(self.get_shape(), other.get_shape());
        let mut out = Self::zeros_like(self);
        for i in 0..self.get_data().len() {
            out.get_data()[i] = self.get_data()[i].clone() + other.get_data()[i].clone();
        }
        out
    }

    fn scalar_add(&self, other: &T) -> Self::ArrayType {
        let mut out = Self::zeros_like(self);
        for i in 0..self.get_data().len() {
            out.get_data()[i] = self.get_data()[i].clone() + other.clone();
        }
        out
    }

    fn inner_product(&self, other: &Self::ArrayType) -> Self::ArrayType {
        assert_eq!(self.get_shape().last().unwrap(), &other.get_shape()[0]);
        // Iterate over the first dim 
        let a = &self.get_shape()[0..self.get_shape().len()-1];
        let b = &other.get_shape()[1..];
        let new_shape: Vec<usize> = [a, b].concat();

        let mut out = Self::zeros(&new_shape);
        out
    }
}

// Impl dot product
impl<'a, T: num_traits::Num + std::clone::Clone> Mul<&'a Array<T>> for Array<T> {
    type Output = Array<T>;
    fn mul(self, other: &Array<T>) -> Array<T> {
        self.inner_product(other)
    }
}
