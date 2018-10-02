extern crate rand;
pub mod base;
pub mod array;

#[cfg(test)]
mod tests {
    use super::base::array::*;

    #[test]
    fn test_size_2d() {
        let w: usize = 1;
        let h: usize = 2;
        let t: Array<f64> = Array::zeros(vec![w,h]);
        for i in 0..w {
            for j in 0..h {
                assert_eq!(t[vec![i, j]], 0.0 as f64);
            }
        }
    }

    #[test]
    fn test_size_3d() {
        let w: usize = 1;
        let h: usize = 2;
        let d: usize = 3;
        let t: Array<f64> = Array::zeros(vec![w,h,d]);
        for i in 0..w {
            for j in 0..h {
                for k in 0..d {
                    assert_eq!(t[vec![i, j, k]], 0.0 as f64);
                }
            }
        }
    }

    #[test]
    fn test_eye() {
        let w: usize = 4;
        let t: Array<f64> = Array::eye(w);
        for i in 0..w {
            for j in 0..w {
                if i == j {
                    assert_eq!(t[vec![i, j]], 1.0 as f64);
                } else {
                    assert_eq!(t[vec![i, j]], 0.0 as f64);
                }
            }
        }
    }

    #[test]
    fn test_ele_index() {
        // test size
        let w: usize = 5;
        for i in 0..6 {
            let m: Array<f64> = Array::ones(vec![w, i]);
            for j in 0..m.len() {
                let v = m.ele_index_inv(j);
                assert_eq!(m.ele_index(v), j);
            }
        }
    }

    extern crate std;
    #[test]
    fn test_slice() {
        let w: usize = 4;
        let t: Array<f64> = Array::eye(w);
        let slices = vec![0..4, 0..1];
        let r = t.slice(slices);
        assert_eq!(r.shape, vec![4, 1]);
        for i in 0..4 {
            assert_eq!(r[vec![i]], t[vec![i, 0]]);
        }
    }

    /*
    #[test]
    fn test_ref_mut() {
        let w: usize = 4;
        let v: usize = 0;
        let new_val = 9 as f64;
        let p = vec![v, v];
        let mut t: array::Array<f64> = array::Array::eye(w);
        let slices = vec![0..4, 0..1];
        {
            let mut r = t.slice_mut(&slices);
            std::mem::replace(r.get_mut(&p), new_val);
        }
        //t.get_mut(&p) = &mut(1 as f64);
        assert_eq!(t.get(&p), &new_val);
    }

    #[test]
    fn test_ref_slice_mut() {
        let w: usize = 4;
        let mut t: array::Array<f64> = array::Array::eye(w);
        let slices = vec![0..4, 0..1];
        let replacement: array::Array<f64> = array::Array::ones(&vec![w, 1]);
        {
            let mut r = t.slice_mut(&slices);
            r.set(replacement.clone());
        }
        for i in 0..w {
            assert_eq!(t.get(&vec![i, 0]), replacement.get(&vec![i, 0]));
        }
    }
    */

    #[test]
    fn test_inner_product() {
        // test size
        let mut m1: Array<f64> = Array::ones(vec![4, 5]);
        let m2: Array<f64> = Array::ones(vec![5, 3]);
        m1.set_subset(vec![0..1, 0..5], 0 as f64);
        let out = m1.inner_product(&m2);
        //println!("m1{:?}", m1);
        //println!("m2{:?}", m2);
        //println!("out{:?}", out);
        // TODO Write sophisticated test. Seems to work fine for now though
    }

    #[test]
    fn test_rand() {
        // test size
        let m: Array<f64> = Array::rand_uniform(vec![4, 5], -1.0, 1.0);
        //println!("m{:?}", m);
    }
}
