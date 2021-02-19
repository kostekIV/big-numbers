use crate::algorithms::karatsuba;
use crate::asm_ops::wrapped_ops;
use crate::asm_ops::{add_const, cmp_slices, div_const, mul_const, sub_const, sub_two_slices};
use crate::errors::ArithmeticError;
use crate::utils::{bit_len, trim_zeros};

const KARATSUBA_THRESHOLD: usize = 13;

#[inline]
fn mul_helper(left: &[u64], right: &[u64], base: u64) -> Vec<u64> {
    let (m, n) = (left.len(), right.len());
    if m == 1 && left[0] == 1 {
        return right.to_vec();
    }

    if n == 1 && right[0] == 1 {
        return left.to_vec();
    }

    if n == 0 {
        return Vec::new();
    }

    let mut repr = vec![0; m + n];

    wrapped_ops::unsafe_mul_two_slices(&left, &right, &mut repr, base);

    while let Some(v) = repr.last() {
        if *v == 0 {
            repr.pop();
        } else {
            break;
        }
    }
    repr
}

pub(crate) fn add(left: &[u64], right: &[u64], base: u64) -> Vec<u64> {
    let mut dst = Vec::new();
    let l: &[u64];
    let r: &[u64];

    if left.len() > right.len() {
        l = left;
        r = right;
    } else {
        l = right;
        r = left;
    }

    let size = usize::max(l.len(), r.len());
    dst.resize(size + 1, 0);

    wrapped_ops::unsafe_add_two_slices(&l, &r, &mut dst, base);
    trim_zeros(&mut dst);
    dst
}

#[allow(clippy::comparison_chain)]
pub(crate) fn sub(left: &[u64], right: &[u64], base: u64) -> (i8, Vec<u64>) {
    let mut dst = Vec::new();
    let mut sign = 0;
    let mut l = left;
    let mut r = right;

    if left.len() > right.len() {
        l = left;
        r = right;
        sign = 1;
    } else if left.len() < right.len() {
        l = right;
        r = left;
        sign = -1;
    } else {
        for i in (0..left.len()).rev() {
            if left[i] > right[i] {
                l = &left[0..=i];
                r = &right[0..=i];
                sign = 1;
                break;
            } else if left[i] < right[i] {
                l = &right[0..=i];
                r = &left[0..=i];
                sign = -1;
                break;
            }
        }

        if sign == 0 {
            return (0, dst);
        }
    }

    let size = usize::max(l.len(), r.len());
    dst.resize(size, 0);

    wrapped_ops::unsafe_sub_two_slices(&l, &r, &mut dst, base);
    trim_zeros(&mut dst);

    if dst.is_empty() {
        sign = 0;
    }

    (sign, dst)
}

#[inline]
pub(crate) fn base_mul(left: &[u64], right: &[u64], base: u64) -> Vec<u64> {
    if left.len() > right.len() {
        mul_helper(left, right, base)
    } else {
        mul_helper(right, left, base)
    }
}

pub(crate) fn mul(left: &[u64], right: &[u64], base: u64) -> Vec<u64> {
    karatsuba(left, right, base, KARATSUBA_THRESHOLD, base_mul)
}

pub(crate) fn div(
    left: &[u64],
    right: &[u64],
    base: u64,
) -> Result<(Vec<u64>, Vec<u64>), ArithmeticError> {
    if right.is_empty() {
        return Err(ArithmeticError::DividedByZero);
    }

    if right.len() == 1 && right[0] == 1 {
        return Ok((left.to_vec(), Vec::from([])));
    } else if right.len() == 1 {
        let mut l = left.to_vec();
        l.reverse();
        let remainder = unsafe { div_const(l.as_mut_ptr(), right[0], base, l.len() as u64) };
        l.reverse();

        let remainder_repr = if remainder == 0 {
            Vec::from([])
        } else {
            Vec::from([remainder])
        };

        return Ok((l, remainder_repr));
    } else if right.len() > left.len() {
        return Ok((Vec::from([]), left.to_vec()));
    }

    // Knuth The art of Computer Programming vol2 3rd edition 4.3.1 Algorithm D
    // Poorly written by me :(.
    let (m, n) = (left.len(), right.len());
    let (mut u, mut v) = (left.to_vec(), right.to_vec());
    u.push(0);
    let mut q = vec![0; m - n + 1];

    let mut r = vec![0; 2];
    let mut br_u = vec![0; 2];
    let mut qp = vec![0; 2];
    let mut qp_copy = vec![0; 3];
    let mut vq;

    let d = u64::pow(2, bit_len(base) - bit_len(v[n - 1]) - 1);

    unsafe {
        mul_const(u.as_mut_ptr(), d, base, (u.len() - 1) as u64);
        mul_const(v.as_mut_ptr(), d, base, v.len() as u64);
    }

    let (vn_1, vn_2) = (v[n - 1], v[n - 2]);

    for j in (0..(m - n + 1)).rev() {
        qp[1] = u[j + n - 1];
        qp[0] = u[j + n];
        let remainder = unsafe { div_const(qp.as_mut_ptr(), vn_1, base, 2) };

        qp.reverse();

        r[0] = remainder;
        r[1] = 0;

        loop {
            if qp == [0, 1] {
                qp[0] = base - 1;
                qp[1] = 0;
                unsafe { add_const(r.as_mut_ptr(), vn_1, base) }
            } else {
                qp_copy[0] = qp[0];
                qp_copy[1] = qp[1];
                qp_copy[2] = 0;

                br_u[0] = u[j + n - 2];
                br_u[1] = r[0];

                unsafe { mul_const(qp_copy.as_mut_ptr(), vn_2, base, 2) };

                if qp_copy[2] != 0 || unsafe { cmp_slices(qp_copy.as_ptr(), br_u.as_ptr(), 2) } == 1
                {
                    unsafe {
                        sub_const(qp.as_mut_ptr(), 1, base);
                        add_const(r.as_mut_ptr(), vn_1, base);
                    }
                } else {
                    break;
                }
            }

            if r[1] != 0 {
                break;
            }
        }

        vq = v.to_vec();
        vq.push(0);

        if !qp.is_empty() {
            unsafe {
                mul_const(vq.as_mut_ptr(), qp[0], base, n as u64);
                if cmp_slices(vq.as_ptr(), u.as_ptr().offset(j as isize), (n + 1) as u64) == 1 {
                    sub_two_slices(
                        vq.as_ptr(),
                        v.as_ptr(),
                        vq.as_mut_ptr(),
                        base,
                        (n + 1) as u64,
                        (n + 1) as u64,
                    );
                    qp[0] -= 1;
                }
                sub_two_slices(
                    u.as_ptr().offset(j as isize),
                    vq.as_ptr(),
                    u.as_mut_ptr().offset(j as isize),
                    base,
                    (n + 1) as u64,
                    (n + 1) as u64,
                );
            }
            q[j] = qp[0];
        } else {
            q[j] = 0;
        }
    }

    u.reverse();
    unsafe {
        div_const(
            u.as_mut_ptr().offset((m - n + 1) as isize),
            d,
            base,
            n as u64,
        );
    }
    u.reverse();

    trim_zeros(&mut q);
    trim_zeros(&mut u);
    return Ok((q.to_vec(), u.to_vec()));
}

pub(crate) fn new_repr(value: u64, base: u64) -> Vec<u64> {
    let mut repr = Vec::new();
    let mut value = value;
    while value >= base {
        repr.push(value % base);
        value /= base;
    }
    if value > 0 {
        repr.push(value);
    }

    repr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_reversed_digits_even_sizes() {
        let a = Vec::from([9, 9, 2, 8, 9]);
        let b = Vec::from([1, 2, 2, 8, 9]);

        let c = Vec::from([0, 2, 5, 6, 9, 1]);

        assert_eq!(c, add(&a, &b, 10));
    }

    #[test]
    fn add_reversed_digits_not_even_sizes() {
        let a = Vec::from([9]);
        let b = Vec::from([1, 9, 9, 9, 9]);

        let c = Vec::from([0, 0, 0, 0, 0, 1]);

        assert_eq!(c, add(&a, &b, 10));
    }

    #[test]
    fn sub_reversed_digits_even_sizes() {
        let a = Vec::from([9, 9, 2, 8, 9]);
        let b = Vec::from([1, 2, 2, 8, 9]);

        let c = (1, Vec::from([8, 7]));

        assert_eq!(c, sub(&a, &b, 10));
    }

    #[test]
    fn sub_reversed_digits_not_even_sizes() {
        let a = Vec::from([9]);
        let b = Vec::from([1, 9, 9, 9, 9]);

        let c = (-1, Vec::from([2, 8, 9, 9, 9]));

        assert_eq!(c, sub(&a, &b, 10));
    }

    #[test]
    fn sub_reversed_digits_not_even_sizes2() {
        let a = Vec::from([9, 8, 2]);
        let b = Vec::from([1, 9, 9, 9, 9]);

        let c = (-1, Vec::from([2, 0, 7, 9, 9]));

        assert_eq!(c, sub(&a, &b, 10));
    }

    #[test]
    fn mul_even() {
        let a = Vec::from([2, 2, 2]);
        let b = Vec::from([3]);

        let c = Vec::from([6, 6, 6]);

        assert_eq!(c, mul(&a, &b, 10));
    }

    #[test]
    fn mul_even_larger() {
        let a = Vec::from([2, 2, 2, 4, 5, 6]);
        let b = Vec::from([3]);

        let c = Vec::from([6, 6, 6, 2, 6, 9, 1]);

        assert_eq!(c, mul(&a, &b, 10));
    }

    #[test]
    fn div_by_single() -> Result<(), ArithmeticError> {
        let a = Vec::from([9, 9, 9, 9, 9]);
        let b = Vec::from([3]);

        let c = Vec::from([3, 3, 3, 3, 3]);
        let d: Vec<u64> = Vec::from([]);

        let (q, r) = div(&a, &b, 10)?;

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_by_single_with_remainder() -> Result<(), ArithmeticError> {
        let a = Vec::from([5, 2, 1]);
        let b = Vec::from([4]);

        let c = Vec::from([1, 3, 0]);
        let d = Vec::from([1]);

        let (q, r) = div(&a, &b, 10)?;

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_by_one() -> Result<(), ArithmeticError> {
        let a = Vec::from([0, 0, 0, 0, 1]);
        let b = Vec::from([1]);

        let c = Vec::from([0, 0, 0, 0, 1]);
        let d: Vec<u64> = Vec::from([]);

        let (q, r) = div(&a, &b, 2)?;

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_by_zero() -> Result<(), ArithmeticError> {
        let a = Vec::from([0, 0, 0, 0, 1]);
        let b = Vec::from([]);

        match div(&a, &b, 10) {
            Ok(_) => assert!(false, "Should throw error"),
            Err(_) => assert!(true),
        }

        Ok(())
    }

    #[test]
    fn div_simple_test_1() -> Result<(), ArithmeticError> {
        let a = Vec::from([9, 9, 9]);
        let b = Vec::from([3, 3]);

        let (q, r) = div(&a, &b, 10)?;

        let c = Vec::from([0, 3]);
        let d = Vec::from([9]);

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_simple_test_2() -> Result<(), ArithmeticError> {
        let a = Vec::from([1, 2, 1]);
        let b = Vec::from([1, 1]);

        let (q, r) = div(&a, &b, 10)?;

        let c = Vec::from([1, 1]);
        let d: Vec<u64> = Vec::from([]);

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_simple_test_3() -> Result<(), ArithmeticError> {
        let a = Vec::from([0, 0, 0, 9, 9, 1, 2]);
        let b = Vec::from([4, 3]);

        let (q, r) = div(&a, &b, 10)?;

        let c = Vec::from([6, 7, 6, 4, 6]);
        let d: Vec<u64> = Vec::from([6, 1]);

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }
}
