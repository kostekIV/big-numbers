
#[inline]
fn add_with_base(x: u64, y: u64, base: u64) -> (u64, u64) {
    let value = x  + y;
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

fn add_to(dest: &mut[u64], l: &[u64], r: &[u64], base: u64) {
    let mut carry = 0;

    let size = l.len();
    for i in 0..size {
        if i < l.len() && i < r.len() {
            let (c1, limb) = add_with_base(l[i], r[i], base);
            let (c2, limb) = add_with_base(limb, carry, base);
            dest[i] = limb;
            carry = c1 + c2;
        } else if i >= r.len() {
            let (c, limb) = add_with_base(l[i], carry, base);
            dest[i] = limb;
            carry = c;
        }
    }

    if carry != 0 {
        dest[size] = carry;
    }
}

fn sub_to(dest: &mut[u64], l: &[u64], r: &[u64], base: u64) {
    let mut carry = 0;

    let size = l.len();
    for i in 0..size {
        if i < l.len() && i < r.len() {
            let (c1, limb) = sub_with_base(l[i], r[i], base);
            let (c2, limb) = sub_with_base(limb, carry, base);
            dest[i] = limb;
            carry = c1 + c2;
        } else if i >= r.len() {
            let (c, limb) = sub_with_base(l[i], carry, base);
            dest[i] = limb;
            carry = c;
        }
    }
}

fn mul_helper(left: &[u64], right: &[u64], base: u64) -> Vec<u64> {
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
            repr[i + j] = limb;
            carry = c1 + c2;
        }

        let mut k = 0;
        while carry != 0 {
            let (c, limb) = add_with_base(repr[i + m + k], carry, base);
            repr[i + m + k] = limb;
            carry = c;
            k += 1;
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

pub(crate) fn add(left: &[u64], right: &[u64], base: u64) -> Vec<u64> {
    let mut dst = Vec::new();
    let l: &[u64];
    let r: &[u64];


    if left.len() > right.len() {
        l = left;
        r = right;
    } else {
        l = right;
        r = left;
    }

    let size = usize::max(l.len(), r.len());
    dst.resize(size + 1, 0);
    add_to(&mut dst, l, r, base);

    if *dst.last().unwrap() == 0 {
        dst.pop();
    }
    dst
}

#[allow(clippy::comparison_chain)]
pub(crate) fn sub(left: &[u64], right: &[u64], base: u64) -> (i8, Vec<u64>) {
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
    sub_to(&mut dst, l, r, base);

    while let Some(v) = dst.last() {
        if *v == 0 {
            dst.pop();
        } else {
            break;
        }
    }

    if dst.is_empty() {
        sign = 0;
    }

    (sign, dst)
}

pub(crate) fn mul(left: &[u64], right: &[u64], base: u64) -> Vec<u64> {
    if left.len() > right.len() {
        mul_helper(left, right, base)
    } else {
        mul_helper(right, left, base)
    }
}

pub(crate) fn new_repr(value: u64, base: u64) -> Vec<u64> {
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
    fn sub_reversed_digits_even_sizes() {
        let a = Vec::from([9, 9, 2, 8, 9]);
        let b = Vec::from([1, 2, 2, 8, 9]);

        let c = (1, Vec::from([8, 7]));

        assert_eq!(c, sub(&a, &b, 10));
    }

    #[test]
    fn sub_reversed_digits_not_even_sizes() {
        let a = Vec::from([9]);
        let b = Vec::from([1, 9, 9, 9, 9]);

        let c = (-1, Vec::from([2, 8, 9, 9, 9]));

        assert_eq!(c, sub(&a, &b, 10));
    }

    #[test]
    fn sub_reversed_digits_not_even_sizes2() {
        let a = Vec::from([9, 8, 2]);
        let b = Vec::from([1, 9, 9, 9, 9]);

        let c = (-1, Vec::from([2, 0, 7, 9, 9]));

        assert_eq!(c, sub(&a, &b, 10));
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
