extern crate num_traits;
extern crate std;
use super::basearray::BaseArray;

pub trait IterArray<'a, T: num_traits::Num + std::clone::Clone + 'a> : BaseArray<T> {
}
