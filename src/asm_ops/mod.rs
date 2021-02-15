
extern "C" {
    pub(crate) fn add_two_slices(a: *const u64, b: *const u64, dest: *mut u64, base: u64, n1: u64, n2: u64);
    pub(crate) fn sub_two_slices(a: *const u64, b: *const u64, dest: *mut u64, base: u64, n1: u64, n2: u64);
    pub(crate) fn mul_two_slices(a: *const u64, b: *const u64, dest: *mut u64, base: u64, n1: u64, n2: u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_slices_even_len() {
        let a = [1, 2, 3, 4, 6];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0,0,0,0,0,0];

        let c = [2, 4, 6, 8, 1, 1];
        unsafe {
            add_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 5, 5)
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_adding_slices_not_even_len() {
        let a = [1, 2, 3, 4, 6, 9];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0,0,0,0,0,0,0];

        let c = [2, 4, 6, 8, 1, 0, 1];
        unsafe {
            add_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 6, 5)
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_sub_slices_even_len() {
        let a = [1, 2, 3, 4, 6];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0,0,0,0,0];

        let c = [0, 0, 0, 0, 1];
        unsafe {
            sub_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 5, 5)
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_sub_slices_not_even_len() {
        let a = [1, 2, 3, 4, 3, 9];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0,0,0,0,0,0];

        let c = [0, 0, 0, 0, 8, 8];
        unsafe {
            sub_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 6, 5)
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_not_even_len() {
        let a = [1, 2, 3, 4, 3, 9];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0,0,0,0,0,0,0,0,0,0,0,0];

        let c = [1,4,0,1,5,2,3,5,7,0,5,0];
        unsafe {
            mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 6, 5)
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_even_len() {
        let a = [1, 2, 3, 4, 6];
        let b = [1, 2, 3, 4, 5];
        let mut dest = [0,0,0,0,0,0,0,0,0,0,0];

        let c = [1, 4, 0, 1, 8, 9, 3, 9, 4, 3, 0];
        unsafe {
            mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 5, 5)
        }
        assert_eq!(c, dest);
    }

    #[test]
    fn test_mul_slices_of_nines_even_len() {
        let a = [9, 9, 9, 9, 9];
        let b = [9, 9, 9, 9, 9];
        let mut dest = [0,0,0,0,0,0,0,0,0,0];

        let c = [1, 0, 0, 0, 0, 8, 9, 9, 9, 9];
        unsafe {
            mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), 10, 5, 5)
        }
        assert_eq!(c, dest);
    }


    #[test]
    fn test_mul_almost_overflow() {
        let base = u64::pow(2, 32);
        let a = [base - 1, base - 1, base - 1, base - 1];
        let b = [base - 1, base - 1, base - 1, base - 1];
        let mut dest = [0,0,0,0,0,0,0,0,0];


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
        let mut dest = [0,0,0,0,0,0];


        let c = [base - 1, base - 1, base - 1, base - 1, 0, 0];
        unsafe {
            mul_two_slices(a.as_ptr(), b.as_ptr(), dest.as_mut_ptr(), base, 4, 1);
        }
        assert_eq!(c, dest);
    }

}

