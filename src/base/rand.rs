extern crate num;
extern crate std;
extern crate rand;

use self::num::traits;
use self::num::traits::identities::{one};

use super::array::Array;
use rand::prelude::*;
use rand::distributions::{Standard, Distribution, StandardNormal};

impl<'a, T> Array<T>
    where T: traits::Num + std::clone::Clone + 'a, 
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
        let mut shell = Self::new(shape, one());
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
        let mut shell = Self::new(shape, one());
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

