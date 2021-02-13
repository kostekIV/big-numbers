use crate::bint::Bint;


#[inline]
fn add_with_base(x: u64, y: u64, base: u64) -> (u64, u64) {
    let value = x  + y ;
    let carry = if value >= base {1} else {0};

    (carry, value - base * carry)
}


#[inline]
fn sub_with_base(x: u64, y: u64, base: u64) -> (u64, u64) {
    let carry = x < y;
    let value;
    if carry {
        value = base - y  + x;
    } else {
        value = x  - y;
    }
    let carry = if carry {1} else {0};

    (carry, value)
}


#[inline]
fn mul_with_base(x: u64, y: u64, carry: u64, base: u64) -> (u64, u64) {
    let value: u64 = x * y + carry;
    let carry = value / base;

    (carry, value - base * carry)
}

pub fn add_to(dest: &mut Vec<u64>, src: &Vec<u64>, base: u64) {
    if src.len() == 0 {
        return;
    }

    let size = usize::max(dest.len(), src.len());
    dest.reserve(size);
    let mut carry = 0;

    for i in 0..size {
        if i < dest.len() && i < src.len() {
            let (c1, limb) = add_with_base(dest[i], src[i], base);
            let (c2, limb) = add_with_base(limb, carry, base);
            dest[i] = limb;
            carry = c1 + c2;
        } else if i >= src.len() {
            let (c, limb) = add_with_base(dest[i], carry, base);
            dest[i] = limb;
            carry = c;

            if carry == 0 {
                return;
            }
        } else {
            let (c, limb) = add_with_base(src[i], carry, base);
            dest.push(limb);
            carry = c;
        }
    }

    if carry != 0 {
        dest.push(carry);
    }
}

pub fn add(left: &Vec<u64>, right: &Vec<u64>, base: u64) -> Vec<u64> {
    if left.len() > right.len() {
        let mut dst = left.to_vec();
        add_to(&mut dst, right, base);

        dst
    } else {
        let mut dst = right.to_vec();
        add_to(&mut dst, left, base);

        dst
    }
}

fn mul_helper(left: &Vec<u64>, right: &Vec<u64>, base: u64) -> Vec<u64> {
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

    let mut repr = vec![0; m + n + 1];
    let mut carry = 0;

    for i in 0..n {
        for j in 0..m {
            let (c1, limb) = mul_with_base(left[j], right[i], carry, base);
            let (c2, limb) = add_with_base(repr[i + j], limb, base);
            repr[i +j] = limb;
            carry = c1 + c2;
        }

        let mut k = 0;
        while carry != 0 {
            let (c, limb) = add_with_base(repr[i + m + k], carry, base);
            repr[i + m + k] = limb;
            carry = c;
        }

    }

    while let Some(v) = repr.last() {
        if *v == 0 {
            repr.pop();
        } else {
            break;
        }
    }

    repr
}

pub fn mul(left: &Vec<u64>, right: &Vec<u64>, base: u64) -> Vec<u64> {
    if left.len() > right.len() {
        mul_helper(left, right, base)
    } else {
        mul_helper(right, left, base)
    }
}

pub fn new_repr(value: u64, base: u64) -> Vec<u64> {
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
    fn add_without_carry() {
        fn test_for_base(base: u64, v1: u64, v2: u64) {
            let (carry, value) = add_with_base(v1, v2, base);
            assert_eq!(0, carry);
            assert_eq!(v1 + v2, value);
        }

        for i in 2..1000 {
            test_for_base(i, i - 2, 1);
        }
    }

    #[test]
    fn add_with_carry() {
        fn test_for_base(base: u64, v1: u64, v2: u64) {
            let (carry, value) = add_with_base(v1, v2, base);
            assert_eq!(1, carry);
            assert_eq!(0, value);
        }

        for i in 2..1000 {
            test_for_base(i, i - 2, 2);
        }
    }

    #[test]
    fn sub_without_carry() {
        fn test_for_base(base: u64, v1: u64, v2: u64) {
            let (carry, value) = sub_with_base(v1, v2, base);
            assert_eq!(0, carry);
            assert_eq!(v1 - v2, value);
        }

        test_for_base(2, 1, 0);
        for i in 3..1000 {
            test_for_base(i, i - 2, 1);
        }
    }

    #[test]
    fn sub_with_carry() {
        fn test_for_base(base: u64, v1: u64, v2: u64) {
            let (carry, value) = sub_with_base(v1, v2, base);
            assert_eq!(1, carry);
            assert_eq!(base  + v1 - v2, value);
        }

        test_for_base(2, 0, 1);
        for i in 3..1000 {
            test_for_base(i, 0, i - 2);
        }
    }

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
    fn mul_without_carry() {
        fn test_for_base(base: u64, v1: u64, v2: u64) {
            let (carry, value) = mul_with_base(v1, v2, 0, base);
            assert_eq!(0, carry);
            assert_eq!(v1 * v2, value);
        }

        for i in 2..1000 {
            test_for_base(i, f64::sqrt(i as f64) as u64, f64::sqrt(i as f64) as u64 - 1);
        }
    }

    #[test]
    fn mul_with_carry() {
        fn test_for_base(base: u64, v1: u64, v2: u64, expected_carry: u64) {
            let (carry, value) = mul_with_base(v1, v2, 0, base);
            assert_eq!(expected_carry, carry);
            assert_eq!(v1 * v2 - expected_carry  * base, value);
        }

        for i in 2..1000 {
            test_for_base(i, i - 1, i - 1, i - 2);
        }
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
        let a = Vec::from([2, 2, 2, 4, 5,6]);
        let b = Vec::from([3]);

        let c = Vec::from([6, 6, 6, 2, 6, 9, 1]);

        assert_eq!(c, mul(&a, &b, 10));
    }
}
