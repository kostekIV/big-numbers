
extern "C" {
    pub(crate) fn add_two_slices(a: *const u64, b: *const u64, dest: *mut u64, base: u64, n1: u64, n2: u64);
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
}

