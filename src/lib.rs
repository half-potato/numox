pub mod array;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_2d() {
        let w = 1;
        let h = 2;
        let t: array::Array<f64> = array::Array::zeros(&vec![w,h]);
        for i in 0..w {
            for j in 0..h {
                assert_eq!(t.get(&([i, j] as [usize; 2])), &(0.0 as f64));
            }
        }
    }

    #[test]
    fn test_size_3d() {
        let w = 1;
        let h = 2;
        let d = 3;
        let t: array::Array<f64> = array::Array::zeros(&vec![w,h,d]);
        for i in 0..w {
            for j in 0..h {
                for k in 0..d {
                    assert_eq!(t.get(&([i, j, k] as [usize; 3])), &(0.0 as f64));
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
                    assert_eq!(t.get(&[i, j]), &(1.0 as f64));
                } else {
                    assert_eq!(t.get(&[i, j]), &(0.0 as f64));
                }
            }
        }
    }
}
