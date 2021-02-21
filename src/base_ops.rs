use crate::IntLimb;
use crate::algorithms::karatsuba;
use crate::asm_ops::wrapped_ops;
use crate::asm_ops::{add_const, cmp_slices, div_const, mul_const, sub_const, sub_two_slices};
use crate::errors::ArithmeticError;
use crate::utils::{bit_len, trim_zeros};

const KARATSUBA_THRESHOLD: usize = 13;

#[inline]
fn mul_helper(left: &[IntLimb], right: &[IntLimb]) -> Vec<IntLimb> {
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

    wrapped_ops::unsafe_mul_two_slices(&left, &right, &mut repr);

    while let Some(v) = repr.last() {
        if *v == 0 {
            repr.pop();
        } else {
            break;
        }
    }
    repr
}

pub(crate) fn add(left: &[IntLimb], right: &[IntLimb]) -> Vec<IntLimb> {
    let mut dst = Vec::new();
    let l: &[IntLimb];
    let r: &[IntLimb];

    if left.len() > right.len() {
        l = left;
        r = right;
    } else {
        l = right;
        r = left;
    }

    let size = usize::max(l.len(), r.len());
    dst.resize(size + 1, 0);

    wrapped_ops::unsafe_add_two_slices(&l, &r, &mut dst);
    trim_zeros(&mut dst);
    dst
}

#[allow(clippy::comparison_chain)]
pub(crate) fn sub(left: &[IntLimb], right: &[IntLimb]) -> (i8, Vec<IntLimb>) {
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

    wrapped_ops::unsafe_sub_two_slices(&l, &r, &mut dst);
    trim_zeros(&mut dst);

    if dst.is_empty() {
        sign = 0;
    }

    (sign, dst)
}

#[inline]
pub(crate) fn base_mul(left: &[IntLimb], right: &[IntLimb]) -> Vec<IntLimb> {
    if left.len() > right.len() {
        mul_helper(left, right)
    } else {
        mul_helper(right, left)
    }
}

pub(crate) fn mul(left: &[IntLimb], right: &[IntLimb]) -> Vec<IntLimb> {
    karatsuba(left, right, KARATSUBA_THRESHOLD, base_mul)
}

pub(crate) fn div(left: &[IntLimb], right: &[IntLimb]) -> Result<(Vec<IntLimb>, Vec<IntLimb>), ArithmeticError> {
    if right.is_empty() {
        return Err(ArithmeticError::DividedByZero);
    }

    if right.len() == 1 && right[0] == 1 {
        return Ok((left.to_vec(), Vec::from([])));
    } else if right.len() == 1 {
        let mut l = left.to_vec();
        l.reverse();
        let remainder = unsafe { div_const(l.as_mut_ptr(), right[0], l.len() as IntLimb) };
        l.reverse();

        let remainder_repr = if remainder == 0 {
            Vec::from([])
        } else {
            Vec::from([remainder])
        };
        trim_zeros(&mut l);
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

    let d = IntLimb::pow(2, bit_len(IntLimb::MAX) - bit_len(v[n - 1]));

    unsafe {
        mul_const(u.as_mut_ptr(), d, (u.len() - 1) as IntLimb);
        mul_const(v.as_mut_ptr(), d, v.len() as IntLimb);
    }
    let (vn_1, vn_2) = (v[n - 1], v[n - 2]);

    for j in (0..(m - n + 1)).rev() {
        qp[1] = u[j + n - 1];
        qp[0] = u[j + n];

        let remainder = unsafe { div_const(qp.as_mut_ptr(), vn_1, 2) };
        qp.reverse();

        r[0] = remainder;
        r[1] = 0;

        loop {
            if qp == [0, 1] {
                qp[0] = IntLimb::MAX;
                qp[1] = 0;
                unsafe { add_const(r.as_mut_ptr(), vn_1) }
            } else {
                qp_copy[0] = qp[0];
                qp_copy[1] = qp[1];
                qp_copy[2] = 0;

                br_u[0] = u[j + n - 2];
                br_u[1] = r[0];

                unsafe { mul_const(qp_copy.as_mut_ptr(), vn_2, 2) };

                if qp_copy[2] != 0 || unsafe { cmp_slices(qp_copy.as_ptr(), br_u.as_ptr(), 2) } == 1
                {
                    unsafe {
                        sub_const(qp.as_mut_ptr(), 1);
                        add_const(r.as_mut_ptr(), vn_1);
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
                mul_const(vq.as_mut_ptr(), qp[0], n as IntLimb);
                if cmp_slices(vq.as_ptr(), u.as_ptr().offset(j as isize), (n + 1) as IntLimb) == 1 {
                    sub_two_slices(
                        vq.as_ptr(),
                        v.as_ptr(),
                        vq.as_mut_ptr(),
                        (n + 1) as IntLimb,
                        (n + 1) as IntLimb,
                    );
                    qp[0] -= 1;
                }
                sub_two_slices(
                    u.as_ptr().offset(j as isize),
                    vq.as_ptr(),
                    u.as_mut_ptr().offset(j as isize),
                    (n + 1) as IntLimb,
                    (n + 1) as IntLimb,
                );
                trim_zeros(&mut vq);
            }
            q[j] = qp[0];
        } else {
            q[j] = 0;
        }
    }

    u.reverse();
    unsafe {
        div_const(u.as_mut_ptr().offset((m - n + 1) as isize), d, n as IntLimb);
    }
    u.reverse();

    trim_zeros(&mut q);
    trim_zeros(&mut u);
    return Ok((q.to_vec(), u.to_vec()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_reversed_digits_even_sizes() {
        let a = Vec::from([9, 9, 2, 8, 9]);
        let b = Vec::from([1, 2, 2, 8, 9]);

        let c = Vec::from([10, 11, 4, 16, 18]);

        assert_eq!(c, add(&a, &b));
    }

    #[test]
    fn add_reversed_digits_not_even_sizes() {
        let a = Vec::from([9]);
        let b = Vec::from([1, 9, 9, 9, 9]);

        let c = Vec::from([10, 9, 9, 9, 9]);

        assert_eq!(c, add(&a, &b));
    }

    #[test]
    fn sub_reversed_digits_even_sizes() {
        let a = Vec::from([9, 9, 2, 8, 9]);
        let b = Vec::from([1, 2, 2, 8, 9]);

        let c = (1, Vec::from([8, 7]));

        assert_eq!(c, sub(&a, &b));
    }

    #[test]
    fn sub_reversed_digits_not_even_sizes() {
        let x = IntLimb::MAX;
        let a = Vec::from([9]);
        let b = Vec::from([1, 9, 9, 9, 9]);

        let c = (-1, Vec::from([x - 7, 8, 9, 9, 9]));

        assert_eq!(c, sub(&a, &b));
    }

    #[test]
    fn sub_reversed_digits_not_even_sizes2() {
        let x = IntLimb::MAX;
        let a = Vec::from([9, 8, 2]);
        let b = Vec::from([1, 9, 9, 9, 9]);

        let c = (-1, Vec::from([x - 7, 0, 7, 9, 9]));

        assert_eq!(c, sub(&a, &b));
    }

    #[test]
    fn mul_even() {
        let a = Vec::from([2, 2, 2]);
        let b = Vec::from([3]);

        let c = Vec::from([6, 6, 6]);

        assert_eq!(c, mul(&a, &b));
    }

    #[test]
    fn mul_even_larger() {
        let a = Vec::from([2, 2, 2, 4, 5, 6]);
        let b = Vec::from([3]);

        let c = Vec::from([6, 6, 6, 12, 15, 18]);

        assert_eq!(c, mul(&a, &b));
    }

    #[test]
    fn div_by_single() -> Result<(), ArithmeticError> {
        let a = Vec::from([9, 9, 9, 9, 9]);
        let b = Vec::from([3]);

        let c = Vec::from([3, 3, 3, 3, 3]);
        let d: Vec<IntLimb> = Vec::from([]);

        let (q, r) = div(&a, &b)?;

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_by_single_with_remainder() -> Result<(), ArithmeticError> {
        let a = Vec::from([5, 2, 1]);
        let b = Vec::from([4]);

        let c = [9223372036854775809, 4611686018427387904].to_vec();
        let d = Vec::from([1]);

        let (q, r) = div(&a, &b)?;

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_by_one() -> Result<(), ArithmeticError> {
        let a = Vec::from([0, 0, 0, 0, 1]);
        let b = Vec::from([1]);

        let c = Vec::from([0, 0, 0, 0, 1]);
        let d: Vec<IntLimb> = Vec::from([]);

        let (q, r) = div(&a, &b)?;

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_by_zero() -> Result<(), ArithmeticError> {
        let a = Vec::from([0, 0, 0, 0, 1]);
        let b = Vec::from([]);

        match div(&a, &b) {
            Ok(_) => assert!(false, "Should throw error"),
            Err(_) => assert!(true),
        }

        Ok(())
    }

    #[test]
    fn div_simple_test_1() -> Result<(), ArithmeticError> {
        let a = Vec::from([9, 9, 9]);
        let b = Vec::from([3, 3]);

        let (q, r) = div(&a, &b)?;

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

        let (q, r) = div(&a, &b)?;

        let c = Vec::from([1, 1]);
        let d: Vec<IntLimb> = Vec::from([]);

        assert_eq!(c, q);
        assert_eq!(d, r);

        Ok(())
    }

    #[test]
    fn div_simple_test_3() -> Result<(), ArithmeticError> {
        let a = Vec::from([0, 0, 0, 9, 9, 1, 2]);
        let b = Vec::from([4, 3]);

        let (q, r) = div(&a, &b)?;

        let c = Vec::from([
            6756214907654938654,
            17991268911395735529,
            9564978408590137872,
            2049638230412172405,
            12297829382473034410,
        ]);
        let d: Vec<IntLimb> = Vec::from([9868628516799348616]);

        assert_eq!(d, r);
        assert_eq!(c, q);

        Ok(())
    }
}
