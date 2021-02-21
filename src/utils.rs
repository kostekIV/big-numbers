use std::cmp::Ordering;

use crate::IntLimb;
use crate::asm_ops::cmp_slices;

#[inline]
pub(crate) fn trim_zeros(x: &mut Vec<IntLimb>) {
    while let Some(v) = x.last() {
        if *v == 0 {
            x.pop();
        } else {
            break;
        }
    }
}

#[inline]
pub(crate) fn bit_len(x: IntLimb) -> u32 {
    let mut i = 0;
    let mut x = x;
    while x > 0 {
        x /= 2;
        i += 1;
    }

    i
}

#[inline]
pub(crate) fn cmp_repr(left: &[IntLimb], right: &[IntLimb]) -> Ordering {
    let (n, m) = (left.len(), right.len());

    match n.cmp(&m) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => {
            let cmp_res = unsafe { cmp_slices(left.as_ptr(), right.as_ptr(), n as IntLimb) };
            match cmp_res {
                -1 => return Ordering::Less,
                0 => return Ordering::Equal,
                1 => return Ordering::Greater,
                _ => panic!("Unexpected result"),
            }
        }
    }
}

#[inline]
pub(crate) fn new_repr(value: IntLimb, base: IntLimb) -> Vec<IntLimb> {
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

#[inline]
pub(crate) fn internal_repr(value: IntLimb) -> Vec<IntLimb> {
    Vec::from([value])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_zeros_empty_vec() {
        let mut empty_vec = vec![0; 0];

        trim_zeros(&mut empty_vec);

        assert_eq!(empty_vec, []);
    }

    #[test]
    fn trim_zeros_zeros_vec_to_empty_vec() {
        let mut zero_vec = vec![100; 0];

        trim_zeros(&mut zero_vec);

        assert_eq!(zero_vec, []);
    }

    #[test]
    fn trim_zeros_zeros_trim_just_zeros() {
        let mut a = [0, 0, 0, 1, 0, 0, 0].to_vec();

        trim_zeros(&mut a);

        assert_eq!(a, [0, 0, 0, 1]);
    }

    #[test]
    fn bit_len_test() {
        let test_inputs = [
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (156, 8),
            (IntLimb::pow(2, 32), 33),
        ];

        for input in test_inputs.iter() {
            assert_eq!(bit_len(input.0), input.1);
        }
    }

    #[test]
    fn cmp_repr_test() {
        let test_inputs = (
            ([1, 2, 3, 4], [], Ordering::Greater),
            ([1, 2, 3, 4], [1, 2], Ordering::Greater),
            ([], [], Ordering::Equal),
            ([1], [5], Ordering::Less),
            ([4], [1, 2], Ordering::Less),
            ([1, 2, 3, 4], [1, 2, 3, 3], Ordering::Greater),
        );

        assert_eq!(
            test_inputs.0 .2,
            cmp_repr(&test_inputs.0 .0, &test_inputs.0 .1)
        );
        assert_eq!(
            test_inputs.1 .2,
            cmp_repr(&test_inputs.1 .0, &test_inputs.1 .1)
        );
        assert_eq!(
            test_inputs.2 .2,
            cmp_repr(&test_inputs.2 .0, &test_inputs.2 .1)
        );
        assert_eq!(
            test_inputs.3 .2,
            cmp_repr(&test_inputs.3 .0, &test_inputs.3 .1)
        );
        assert_eq!(
            test_inputs.4 .2,
            cmp_repr(&test_inputs.4 .0, &test_inputs.4 .1)
        );
        assert_eq!(
            test_inputs.5 .2,
            cmp_repr(&test_inputs.5 .0, &test_inputs.5 .1)
        );
    }
}
