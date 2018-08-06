use super::BaseArray;
extern crate num_traits;
extern crate std;

pub trait MutArray<'a, T> : BaseArray<'a, T>
    where T: num_traits::Num + std::clone::Clone + 'a
{
    fn at_mut(&mut self, ind: &usize) -> &mut T;
    fn get_mut(&mut self, ind: &Vec<usize>) -> &mut T;
    fn set_at(&mut self, ind: &usize, val: &T);
    fn set<R: BaseArray<'a, T>>(&mut self, other: R);
    fn set_scalar(&mut self, other: T);
}
