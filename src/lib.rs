pub mod array;

#[cfg(test)]
mod tests {
    use super::array::*;

    #[test]
    fn test_size_2d() {
        let w: usize = 1;
        let h: usize = 2;
        let t: Array<f64> = Array::zeros(&vec![w,h]);
        for i in 0..w {
            for j in 0..h {
                assert_eq!(t.get(&vec![i, j]), &(0.0 as f64));
            }
        }
    }

    #[test]
    fn test_size_3d() {
        let w: usize = 1;
        let h: usize = 2;
        let d: usize = 3;
        let t: array::Array<f64> = array::Array::zeros(&vec![w,h,d]);
        for i in 0..w {
            for j in 0..h {
                for k in 0..d {
                    assert_eq!(t.get(&vec![i, j, k]), &(0.0 as f64));
                }
            }
        }
    }

    #[test]
    fn test_eye() {
        let w: usize = 4;
        let t: array::Array<f64> = array::Array::eye(w);
        for i in 0..w {
            for j in 0..w {
                if i == j {
                    assert_eq!(t.get(&vec![i, j]), &(1.0 as f64));
                } else {
                    assert_eq!(t.get(&vec![i, j]), &(0.0 as f64));
                }
            }
        }
    }

    /*
    extern crate std;
    #[test]
    fn test_ref() {
        let w: usize = 4;
        let v: usize = 0;
        let new_val = 9 as f64;
        let p = vec![v, v];
        let mut t: array::Array<f64> = array::Array::eye(w);
        {
            let mut r = t.reference();
            std::mem::replace(r.get_mut(&p), new_val);
        }
        //t.get_mut(&p) = &mut(1 as f64);
        assert_eq!(t.get(&p), &new_val);
    }
    */

    #[test]
    fn test_slice() {
        let w: usize = 4;
        let v: usize = 0;
        let t: array::Array<f64> = array::Array::eye(w);
        let slices = vec![0..4, 0..1];
        let r = t.slice(slices);
        assert_eq!(r.get_shape(), &vec![4, 1]);
        println!("{:?}" , r);
        for i in 0..4 {
            assert_eq!(r.at(&i), t.get(&vec![i, 0]));
        }
    }

}
