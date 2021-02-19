extern "C" {
    pub(crate) fn add_two_slices(
        a: *const u64,
        b: *const u64,
        dest: *mut u64,
        base: u64,
        n1: u64,
        n2: u64,
    );
    pub(crate) fn sub_two_slices(
        a: *const u64,
        b: *const u64,
        dest: *mut u64,
        base: u64,
        n1: u64,
        n2: u64,
    );
    pub(crate) fn mul_two_slices(
        a: *const u64,
        b: *const u64,
        dest: *mut u64,
        base: u64,
        n1: u64,
        n2: u64,
    );
}

extern "C" {
    pub(crate) fn add_const(dest: *mut u64, c: u64, base: u64);
    pub(crate) fn sub_const(dest: *mut u64, c: u64, base: u64);
    pub(crate) fn mul_const(dest: *mut u64, c: u64, base: u64, n: u64);
    pub(crate) fn div_const(dest: *mut u64, c: u64, base: u64, n: u64) -> u64;
}

extern "C" {
    pub(crate) fn cmp_slices(a: *const u64, b: *const u64, n: u64) -> i32;
}

pub mod wrapped_ops {
    pub(crate) fn unsafe_add_two_slices(a: &[u64], b: &[u64], d: &mut [u64], base: u64) {
        let (n, m) = (a.len(), b.len());
        assert!(d.len() >= usize::max(n, m) + 1);
        unsafe {
            if n > m {
                super::add_two_slices(
                    a.as_ptr(),
                    b.as_ptr(),
                    d.as_mut_ptr(),
                    base,
                    n as u64,
                    m as u64,
                );
            } else {
                super::add_two_slices(
                    b.as_ptr(),
                    a.as_ptr(),
                    d.as_mut_ptr(),
                    base,
                    m as u64,
                    n as u64,
                );
            }
        }
    }

    pub(crate) fn unsafe_sub_two_slices(a: &[u64], b: &[u64], d: &mut [u64], base: u64) {
        let (n, m) = (a.len(), b.len());
        assert!(d.len() >= usize::max(n, m) && n >= m);
        unsafe {
            super::sub_two_slices(
                a.as_ptr(),
                b.as_ptr(),
                d.as_mut_ptr(),
                base,
                n as u64,
                m as u64,
            );
        }
    }

    pub(crate) fn unsafe_mul_two_slices(a: &[u64], b: &[u64], d: &mut [u64], base: u64) {
        let (n, m) = (a.len(), b.len());
        assert!(d.len() >= n + m);
        unsafe {
            super::mul_two_slices(
                a.as_ptr(),
                b.as_ptr(),
                d.as_mut_ptr(),
                base,
                n as u64,
                m as u64,
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

        let c = [2, 4, 6, 8, 1, 1];
        unsafe { add_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 5, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_adding_slices_not_even_len() {
        let a = [1, 2, 3, 4, 6, 9];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0, 0, 0, 0, 0, 0, 0];

        let c = [2, 4, 6, 8, 1, 0, 1];
        unsafe { add_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 6, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_sub_slices_even_len() {
        let a = [1, 2, 3, 4, 6];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0, 0, 0, 0, 0];

        let c = [0, 0, 0, 0, 1];
        unsafe { sub_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 5, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_sub_slices_not_even_len() {
        let a = [1, 2, 3, 4, 3, 9];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0, 0, 0, 0, 0, 0];

        let c = [0, 0, 0, 0, 8, 8];
        unsafe { sub_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 6, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_not_even_len() {
        let a = [1, 2, 3, 4, 3, 9];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let c = [1, 4, 0, 1, 5, 2, 3, 5, 7, 0, 5, 0];
        unsafe { mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 6, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_even_len() {
        let a = [1, 2, 3, 4, 6];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let c = [1, 4, 0, 1, 8, 9, 3, 9, 4, 3, 0];
        unsafe { mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 5, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_of_nines_even_len() {
        let a = [9, 9, 9, 9, 9];
        let b = [9, 9, 9, 9, 9];
        let mut dest = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let c = [1, 0, 0, 0, 0, 8, 9, 9, 9, 9];
        unsafe { mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 5, 5) }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_almost_overflow() {
        let base = u64::pow(2, 32);
        let a = [base - 1, base - 1, base - 1, base - 1];
        let b = [base - 1, base - 1, base - 1, base - 1];
        let mut dest = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        let c = [1, 0, 0, 0, base - 2, base - 1, base - 1, base - 1, 0];
        unsafe {
            mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), base, 4, 4);
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_by_one() {
        let base = u64::pow(2, 32);
        let a = [base - 1, base - 1, base - 1, base - 1];
        let b = [1];
        let mut dest = [0, 0, 0, 0, 0, 0];

        let c = [base - 1, base - 1, base - 1, base - 1, 0, 0];
        unsafe {
            mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), base, 4, 1);
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_adding_const() {
        let mut a = [9, 9, 9, 9, 9, 0];
        let b = 1;

        let c = [0, 0, 0, 0, 0, 1];
        unsafe {
            add_const(a.as_mut_ptr(), b, 10);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_sub_const() {
        let mut a = [9, 9, 9, 9, 9, 0];
        let b = 1;

        let c = [8, 9, 9, 9, 9, 0];
        unsafe {
            sub_const(a.as_mut_ptr(), b, 10);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_sub_const_overflow() {
        let mut a = [1, 0, 0, 1];
        let b = 2;

        let c = [9, 9, 9, 0];
        unsafe {
            sub_const(a.as_mut_ptr(), b, 10);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_mul_const() {
        let mut a = [9, 9, 9, 9, 9, 0];
        let b = 1;

        let c = [9, 9, 9, 9, 9, 0];
        unsafe {
            mul_const(a.as_mut_ptr(), b, 10, 5);
        }
        assert_eq!(c, a);
    }

    #[test]
    fn test_mul_const_overflow() {
        let mut a = [9, 9, 9, 9, 9, 0];
        let b = 2;

        let c = [8, 9, 9, 9, 9, 1];
        unsafe {
            mul_const(a.as_mut_ptr(), b, 10, 5);
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
        let remainder = unsafe { div_const(a.as_mut_ptr(), 3, 10, 5) };
        assert_eq!(0, remainder);
        assert_eq!(c, a);
    }

    #[test]
    fn test_div_const_2() {
        let mut a = [1, 2, 5];

        let c = [0, 3, 1];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 4, 10, 3) };
        assert_eq!(c, a);
        assert_eq!(1, remainder);
    }

    #[test]
    fn test_div_const_3() {
        let mut a = [9, 9, 9, 9, 9];

        let c = [2, 4, 9, 9, 9];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 4, 10, 5) };
        assert_eq!(c, a);
        assert_eq!(3, remainder);
    }

    #[test]
    fn test_div_const_4() {
        let mut a = [1];

        let c = [0];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 2, 10, 1) };
        assert_eq!(1, remainder);
        assert_eq!(c, a);
    }

    #[test]
    fn test_div_const_5() {
        let mut a = [5];

        let c = [1];
        let remainder = unsafe { div_const(a.as_mut_ptr(), 4, 10, 1) };
        assert_eq!(1, remainder);
        assert_eq!(c, a);
    }
}
