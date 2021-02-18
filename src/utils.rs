#[inline]
pub(crate) fn trim_zeros(x: &mut Vec<u64>) {
    while let Some(v) = x.last() {
        if *v == 0 {
            x.pop();
        } else {
            break;
        }
    }
}

#[inline]
pub(crate) fn bit_len(x: u64) -> u32 {
    let mut i = 0;
    let mut x = x;
    while x > 0 {
        x /= 2;
        i += 1;
    }

    i
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
            (u64::pow(2, 32), 33),
        ];

        for input in test_inputs.iter() {
            assert_eq!(bit_len(input.0), input.1);
        }
    }
}
