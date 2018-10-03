extern crate std;
extern crate num;

use super::array::Array;
use super::sliceiter::IndexIter;

impl<T> Array<T> 
    where T: std::clone::Clone
{
    // IndexIter is an object that can be a range, individual index, or the entire dim
    // Convert list of IndexIter to list of indices for each dim
    pub fn make_slice(&self, index: Vec<IndexIter>) -> Vec<Vec<usize>> {
        let mut slicer: Vec<Vec<usize>> = Vec::new();
        for i in 0..self.shape.len() {
            if i >= index.len() {
                slicer.push((0..self.shape[i]).collect());
            } else {
                slicer.push(match index[i].clone() {
                    IndexIter::Index(ind) => (ind..ind+1).collect(),
                    IndexIter::Range(ref rng) => (rng.clone()).collect(),
                    IndexIter::Whole(_) => (0..self.shape[i]).collect(),
                    IndexIter::Slice(_) => (0..self.shape[i]).collect(),
                });
            }
        }
        slicer
    }

    // Convert list of indices to list of coordinates for elements of new mat + shape
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

    // Convert list of indices to sub matrix
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

    // Convert list of IndexIter to sub matrix
    pub fn subset(&self, index: Vec<IndexIter>) -> Array<T> {
        self.slice(self.make_slice(index))
    }

    // Set list of IndexIter to sub matrix
    pub fn set_subset(&mut self, index: Vec<IndexIter>, val: Array<T>) {
        let (coords, new_shape) = self.slice_to_ind(self.make_slice(index));
        assert_eq!(new_shape, val.shape);
        for (i, coord) in coords.iter().enumerate() {
            self[coord.clone()] = val.data[i].clone();
        }
    }
}
