use crate::asm_ops::div_const;
use crate::base_ops::{add, mul};
use crate::utils::internal_repr;
use crate::IntLimb;

pub(crate) fn convert_to_internal(from: IntLimb, numbers: &[IntLimb]) -> Vec<IntLimb> {
    let mut new_vec = Vec::new();
    let bv = internal_repr(from);

    for number in numbers {
        let v = internal_repr(*number);
        new_vec = add(&mul(&new_vec, &bv), &v);
    }

    new_vec
}

pub(crate) fn convert_from_internal(to: IntLimb, numbers: &[IntLimb]) -> Vec<IntLimb> {
    let mut nrs = numbers.to_vec();
    let mut repr = Vec::new();

    while !nrs.is_empty() {
        let remainder = unsafe { div_const(nrs.as_mut_ptr(), to, nrs.len() as IntLimb) };
        repr.push(remainder);
        let mut i = 0;
        for x in nrs.iter() {
            if *x == 0 {
                i += 1;
            } else {
                break;
            }
        }
        if i > 0 {
            nrs.drain(0..i);
        }
    }

    repr
}

pub(crate) fn convert_from_string(from: IntLimb, number: String) -> Vec<IntLimb> {
    let numbers: Vec<_> = number
        .chars()
        .map(|x| x.to_digit(from as u32))
        .flatten()
        .map(|x| x as IntLimb)
        .collect();
    assert!(numbers.len() == number.len());

    convert_to_internal(from, &numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_base() {
        let number = "10".to_string();

        let r = convert_from_string(10, number);

        assert_eq!(vec![10], r);
    }

    #[test]
    fn smaller_to_larger() {
        let number = "1111111".to_string();

        let r = convert_from_string(2, number);

        assert_eq!(vec![127], r);
    }

    #[test]
    fn smaller_to_much_larger() {
        let number = "100000".to_string();

        let r = convert_from_string(10, number);

        assert_eq!(vec![100000], r);
    }

    #[test]
    fn zero() {
        let number = "00000000000".to_string();

        let r = convert_from_string(10, number);

        assert_eq!(vec![0; 0], r);
    }
}
