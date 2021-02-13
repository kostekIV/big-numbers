use crate::base_ops::{add_to, mul, new_repr};

pub fn convert(from: u64, to: u64, numbers: &Vec<u64>) -> Vec<u64> {
    let mut new_vec = Vec::new();
    let bv = new_repr(from, to);

    for number in numbers {
        let v = new_repr(*number, to);
        new_vec = mul(&new_vec, &bv, to);
        add_to(&mut new_vec, &v, to);
    }

    return new_vec;
}

pub fn convert_from_string(from: u64, to: u64, number: String) -> Vec<u64> {
    let numbers: Vec<_> = number
            .chars()
            .map(|x| x.to_digit(from as u32))
            .flatten()
            .map(|x| x as u64)
            .collect();
    assert!(numbers.len() == number.len());

    convert(from, to, &numbers)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_base() {
        let number = "10".to_string();

        let r = convert_from_string(10, 2, number);

        assert_eq!(vec![0, 1, 0, 1], r);
    }


    #[test]
    fn smaller_to_larger() {
        let number = "1111111".to_string();

        let r = convert_from_string(2, 3, number);

        assert_eq!(vec![1, 0, 2, 1, 1], r);
    }


    #[test]
    fn smaller_to_much_larger() {
        let number = "100000".to_string();

        let r = convert_from_string(10, 1000000000, number);

        assert_eq!(vec![100000], r);
    }

    #[test]
    fn larger_to_smaller() {
        let number = vec![286, 3051653344];

        let r = convert(4294967296, 10, &number);

        assert_eq!(vec![0, 0, 0, 0, 0, 3, 2, 1, 4, 1, 3, 2, 1], r);
    }

    #[test]
    fn zero() {
        let number = "00000000000".to_string();

        let r = convert_from_string(10, 1000, number);

        assert_eq!(vec![0; 0], r);
    }
}

