extern crate num_traits;
extern crate std;

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
        assert_eq!(ind.len(), self.get_shape().len(), "Tried to index array using wrong number of dimensions");
        let mut index: usize = 0;
        for i in 0..ind.len() {
            assert!(ind[i] < self.get_shape()[i], "Index exceeds dimension of array");
            let axis_val: usize = (0..i).map(|x| self.get_shape()[x]).product();
            index += ind[i]*axis_val;
        }
        index
    }

    fn get_shape_mut(&mut self) -> &mut Vec<usize>;

    fn ele_index_inv(&self, ind: &usize) -> Vec<usize> {
        assert!(ind < &self.size());
        let mut index: Vec<usize> = vec![0; self.get_shape().len()];
        let mut ind2 = ind.clone();
        for i in 0..self.get_shape().len() {
            if ind2 == 0{
                break;
            }
            // Index in reverse
            let j = self.get_shape().len() - i - 1;
            // 
            let place_val: usize =
                (0..j)
                .map(|x| self.get_shape()[x])
                .product();
            index[j] = ((ind2 as f64)/(place_val as f64)).floor() as usize;
            ind2 = ind2%place_val;
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
    fn iter(&'a self) -> Iter<'a, T, Self::ArrayType>;

    fn slice<R>(&'a self, slices: &Vec<R>) -> RefArray<'a, T>
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
