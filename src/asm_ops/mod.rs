use crate::IntLimb;

extern "C" {
    pub(crate) fn add_two_slices(
        a: *const IntLimb,
        b: *const IntLimb,
        dest: *mut IntLimb,
        n1: IntLimb,
        n2: IntLimb,
    );
    pub(crate) fn sub_two_slices(
        a: *const IntLimb,
        b: *const IntLimb,
        dest: *mut IntLimb,
        n1: IntLimb,
        n2: IntLimb,
    );
    pub(crate) fn mul_two_slices(
        a: *const IntLimb,
        b: *const IntLimb,
        dest: *mut IntLimb,
        n1: IntLimb,
        n2: IntLimb,
    );
}

extern "C" {
    pub(crate) fn add_const(dest: *mut IntLimb, c: IntLimb);
    pub(crate) fn sub_const(dest: *mut IntLimb, c: IntLimb);
    pub(crate) fn mul_const(dest: *mut IntLimb, c: IntLimb, n: IntLimb);
    pub(crate) fn div_const(dest: *mut IntLimb, c: IntLimb, n: IntLimb) -> IntLimb;
}

extern "C" {
    pub(crate) fn cmp_slices(a: *const IntLimb, b: *const IntLimb, n: IntLimb) -> i32;
}

pub mod wrapped_ops {
    use super::*;

    pub(crate) fn unsafe_add_two_slices(a: &[IntLimb], b: &[IntLimb], d: &mut [IntLimb]) {
        let (n, m) = (a.len(), b.len());
        assert!(d.len() >= usize::max(n, m) + 1);
        unsafe {
            if n > m {
                super::add_two_slices(
                    a.as_ptr(),
                    b.as_ptr(),
                    d.as_mut_ptr(),
                    n as IntLimb,
                    m as IntLimb,
                );
            } else {
                super::add_two_slices(
                    b.as_ptr(),
                    a.as_ptr(),
                    d.as_mut_ptr(),
                    m as IntLimb,
                    n as IntLimb,
                );
            }
        }
    }

    pub(crate) fn unsafe_sub_two_slices(a: &[IntLimb], b: &[IntLimb], d: &mut [IntLimb]) {
        let (n, m) = (a.len(), b.len());
        assert!(d.len() >= usize::max(n, m) && n >= m);
        unsafe {
            super::sub_two_slices(
                a.as_ptr(),
                b.as_ptr(),
                d.as_mut_ptr(),
                n as IntLimb,
                m as IntLimb,
            );
        }
    }

    pub(crate) fn unsafe_mul_two_slices(a: &[IntLimb], b: &[IntLimb], d: &mut [IntLimb]) {
        let (n, m) = (a.len(), b.len());
        assert!(d.len() >= n + m);
        unsafe {
            super::mul_two_slices(
                a.as_ptr(),
                b.as_ptr(),
                d.as_mut_ptr(),
                n as IntLimb,
                m as IntLimb,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_slices_even_len() {
        let a = [1, 2, 3, 4, 6];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0, 0, 0, 0, 0, 0];

        let c = [2, 4, 6, 8, 11, 0];
        unsafe { add_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 5, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_adding_slices_not_even_len() {
        let a = [1, 2, 3, 4, 6, 9];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0, 0, 0, 0, 0, 0, 0];

        let c = [2, 4, 6, 8, 11, 9, 0];
        unsafe { add_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 6, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_adding_slices_overflow() {
        let x = IntLimb::MAX;
        let a = [x, x, x, 0, x];
        let b = [1, 0, 0, x];
        let mut dest = [0, 0, 0, 0, 0, 0];

        let c = [0, 0, 0, 0, 0, 1];
        unsafe { add_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 6, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_sub_slices_even_len() {
        let x = IntLimb::MAX;
        let a = [0, 0, 0, 0, 0, 2];
        let b = [1, 1, 0, 0, 0, 1];
        let mut dest = [0, 0, 0, 0, 0, 0];

        let c = [x, x - 1, x, x, x, 0];
        unsafe { sub_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 6, 6) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_sub_slices_not_even_len() {
        let x = IntLimb::MAX;
        let a = [0, 1, 2, 1, 0, 1, 1];
        let b = [1, 0, 3];
        let mut dest = [0, 0, 0, 0, 0, 0, 0];

        let c = [x, 0, x, 0, 0, 1, 1];
        unsafe { sub_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 7, 3) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_sub_slices_to_zeros() {
        let a = [1, 1, 1, 1, 1, 1];
        let b = [1, 1, 1, 1, 1, 1];
        let mut dest = [0, 0, 0, 0, 0, 0];

        let c = [0, 0, 0, 0, 0, 0];
        unsafe { sub_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 6, 6) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_not_even_len() {
        let x = IntLimb::MAX;
        let a = [x, x, x];
        let b = [2, 1];
        let mut dest = [0, 0, 0, 0, 0];
        let c = [x - 1, x - 1, x, 1, 1];
        unsafe { mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 3, 2) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_even_len() {
        let a = [1, 2, 3, 4, 6];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        // let c = [1, 2, 3, 4, 6];
        // let c =    [2, 4, 6, 8, 12];
        // let c =       [3, 6, 9, 12, 18];
        // let c =          [4, 8, 12, 16, 24];
        // let c =             [5, 10, 15, 20, 30];
        let c = [1, 4, 10, 20, 36, 46, 49, 44, 30, 0];
        unsafe { mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 5, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_of_nines_even_len() {
        let x = IntLimb::MAX;
        let a = [x, x, x, x, x];
        let b = [x, x, x, x, x];
        let mut dest = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let c = [1, 0, 0, 0, 0, x - 1, x, x, x, x];
        unsafe { mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 5, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_by_one() {
        let x = IntLimb::pow(2, 32);
        let a = [x - 1, x - 1, x - 1, x - 1];
        let b = [1];
        let mut dest = [0, 0, 0, 0, 0, 0];

        let c = [x - 1, x - 1, x - 1, x - 1, 0, 0];
        unsafe {
            mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 4, 1);
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_adding_const() {
        let x = IntLimb::MAX;
        let mut a = [x, x, x, 0];
        let b = 1;

        let c = [0, 0, 0, 1];
        unsafe {
            add_const(a.as_mut_ptr(), b);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_sub_const() {
        let x = IntLimb::MAX;
        let mut a = [0, 0, 0, 1];
        let b = 1;

        let c = [x, x, x, 0];
        unsafe {
            sub_const(a.as_mut_ptr(), b);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_sub_const_overflow() {
        let x = IntLimb::MAX;
        let mut a = [1, 0, 0, 1];
        let b = 2;

        let c = [x, x, x, 0];
        unsafe {
            sub_const(a.as_mut_ptr(), b);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_mul_const_1() {
        let mut a = [9, 9, 9, 9, 9, 0];
        let b = 1;

        let c = [9, 9, 9, 9, 9, 0];
        unsafe {
            mul_const(a.as_mut_ptr(), b, 5);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_mul_const_2() {
        let mut a = [9, 9, 9, 9, 9, 0];
        let b = 2;

        let c = [18, 18, 18, 18, 18, 0];
        unsafe {
            mul_const(a.as_mut_ptr(), b, 5);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_mul_const_3() {
        let x = IntLimb::MAX;
        let mut a = [x, x, x, 0];
        let b = x;

        let c = [1, x, x, x - 1];
        unsafe {
            mul_const(a.as_mut_ptr(), b, 4);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_cmp_slices_1() {
        let a = [1, 2, 3, 4, 6];
        let b = [1, 2, 3, 4, 5];

        let res = unsafe { cmp_slices(a.as_ptr(), b.as_ptr(), 5) };
        assert_eq!(res, 1);
    }

    #[test]
    fn test_cmp_slices_2() {
        let a = [1, 2, 3, 4, 4];
        let b = [1, 2, 3, 4, 5];

        let res = unsafe { cmp_slices(a.as_ptr(), b.as_ptr(), 5) };
        assert_eq!(res, -1);
    }

    #[test]
    fn test_cmp_slices_3() {
        let a = [1, 2, 3, 4, 5];
        let b = [1, 2, 3, 4, 5];

        let res = unsafe { cmp_slices(a.as_ptr(), b.as_ptr(), 5) };
        assert_eq!(res, 0);
    }

    #[test]
    fn test_div_const_1() {
        let mut a = [9, 9, 9, 9, 9];

        let c = [3, 3, 3, 3, 3];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 3, 5) };
        assert_eq!(0, remainder);
        assert_eq!(c, a);
    }

    #[test]
    fn test_div_const_2() {
        let mut a = [1, 2, 5];

        let c = [0, 4611686018427387904, 9223372036854775809];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 4, 3) };
        assert_eq!(1, remainder);
        assert_eq!(c, a);
    }

    #[test]
    fn test_div_const_3() {
        let mut a = [9, 9, 9, 9, 9];

        let c = [
            2,
            4611686018427387906,
            4611686018427387906,
            4611686018427387906,
            4611686018427387906,
        ];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 4, 5) };
        assert_eq!(c, a);
        assert_eq!(1, remainder);
    }

    #[test]
    fn test_div_const_4() {
        let mut a = [1];

        let c = [0];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 2, 1) };
        assert_eq!(1, remainder);
        assert_eq!(c, a);
    }

    #[test]
    fn test_div_const_5() {
        let mut a = [5];

        let c = [1];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 4, 1) };
        assert_eq!(1, remainder);
        assert_eq!(c, a);
    }

    #[test]
    fn test_div_const_6() {
        let mut a = [9223372036854775808, 4611686018427387906];

        let c = [0, 12297829382473034410];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 13835058055282163713, 2) };
        assert_eq!(c, a);
        assert_eq!(1537228672809129304, remainder);
    }
}
