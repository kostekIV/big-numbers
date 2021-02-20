use crate::asm_ops::wrapped_ops;
use crate::asm_ops::{add_two_slices, sub_two_slices};
use crate::utils::trim_zeros;

type MulFunction = fn(&[u64], &[u64]) -> Vec<u64>;

#[inline]
fn add_helper(dest: &mut [u64], a: &[u64], l: u64, offset: isize) {
    if a.is_empty() {
        return;
    }
    unsafe {
        if l > a.len() as u64 {
            add_two_slices(
                dest.as_ptr().offset(offset),
                a.as_ptr(),
                dest.as_mut_ptr().offset(offset),
                a.len() as u64,
                a.len() as u64,
            );
        } else {
            add_two_slices(
                a.as_ptr(),
                dest.as_ptr().offset(offset),
                dest.as_mut_ptr().offset(offset),
                a.len() as u64,
                a.len() as u64,
            );
        }
    }
}

pub(crate) fn karatsuba(
    a: &[u64],
    b: &[u64],
    threshold: usize,
    mul_function: MulFunction,
) -> Vec<u64> {
    let (n, m) = (a.len(), b.len());

    if n <= threshold || m <= threshold {
        return mul_function(&a, &b);
    }

    let l = if n > m { n / 2 } else { m / 2 };

    let mut dest = vec![0; 6 * l + 1];

    let (mut a1, mut a0) = if l >= n {
        ([].to_vec(), a[0..n].to_vec())
    } else {
        (a[l..n].to_vec(), a[0..l].to_vec())
    };

    let (mut b1, mut b0) = if l >= m {
        ([].to_vec(), b[0..m].to_vec())
    } else {
        (b[l..m].to_vec(), b[0..l].to_vec())
    };

    trim_zeros(&mut a1);
    trim_zeros(&mut b1);
    trim_zeros(&mut a0);
    trim_zeros(&mut b0);

    let z0 = karatsuba(&a0, &b0, threshold, mul_function);
    let z2 = karatsuba(&a1, &b1, threshold, mul_function);

    unsafe {
        let mut ab0 = vec![0; usize::max(a0.len(), a1.len()) + 1];
        let mut ab1 = vec![0; usize::max(b0.len(), b1.len()) + 1];

        wrapped_ops::unsafe_add_two_slices(&a0, &a1, &mut ab0);
        wrapped_ops::unsafe_add_two_slices(&b0, &b1, &mut ab1);

        trim_zeros(&mut ab0);
        trim_zeros(&mut ab1);

        let mut z1 = karatsuba(&ab1, &ab0, threshold, mul_function);

        if !z2.is_empty() {
            sub_two_slices(
                z1.as_ptr(),
                z2.as_ptr(),
                z1.as_mut_ptr(),
                z1.len() as u64,
                z2.len() as u64,
            );
        }

        trim_zeros(&mut z1);

        if !z0.is_empty() {
            sub_two_slices(
                z1.as_ptr(),
                z0.as_ptr(),
                z1.as_mut_ptr(),
                z1.len() as u64,
                z0.len() as u64,
            );
        }

        trim_zeros(&mut z1);

        add_helper(&mut dest, &z0, l as u64, 0);
        add_helper(&mut dest, &z1, l as u64, l as isize);
        add_helper(&mut dest, &z2, l as u64, 2 * l as isize);
    }

    trim_zeros(&mut dest);
    dest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base_ops::base_mul;

    #[test]
    fn karatsuba_test_1() {
        let a = Vec::from([1, 1, 1, 1, 1, 1]);
        let b = Vec::from([1, 1, 1, 1, 1, 1]);

        let expected_res = Vec::from([1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]);
        let res = karatsuba(&a, &b, 3, base_mul);

        assert_eq!(expected_res, res);
    }

    #[test]
    fn karatsuba_test_2() {
        let a = Vec::from([6, 5, 0, 7, 4, 5, 1, 1, 5, 9, 2, 7, 2, 0, 3]);
        let b = Vec::from([6, 9, 2, 7]);

        let expected_res = Vec::from([
            36, 84, 57, 94, 122, 80, 108, 53, 76, 108, 110, 113, 142, 46, 71, 41, 6, 21,
        ]);

        let res = karatsuba(&a, &b, 7, base_mul);
        assert_eq!(expected_res, res);
    }
}
