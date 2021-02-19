use crate::asm_ops::{add_two_slices, sub_two_slices, mul_two_slices};
use crate::utils::trim_zeros;

type MulFunction = fn(&[u64], &[u64],  u64) -> Vec<u64>;

pub(crate) fn karatsuba(a: &[u64], b: &[u64], base: u64, threshold: usize, mul_function: MulFunction) -> Vec<u64> {
    let (n, m) = (a.len(), b.len());

    if n < threshold && m < threshold {
        return mul_function(&a, &b, base);
    }

    let l = if n > m {
        n / 2
    } else {
        m / 2
    };

    let mut dest = vec![0; 3 * l];

    let (a1, a0) = if l >= n {
        (a[0..0].to_vec(), a[0..n].to_vec())
    } else {
        (a[l..n].to_vec(), a[0..l].to_vec())
    };

    let (b1, b0) = if l >= m {
        (b[0..0].to_vec(), b[0..m].to_vec())
    } else {
        (b[l..n].to_vec(), b[0..l].to_vec())
    };

    let mut z0 = vec![0; a0.len() + b0.len() + 1];
    let mut z2 = vec![0; a1.len() + b1.len() + 1];

    unsafe {
        mul_two_slices(
            a0.as_ptr(),
            b0.as_ptr(),
            z0.as_mut_ptr(),
            base,
            a0.len() as u64,
            b0.len() as u64,
        );
        if !a1.is_empty() && !b1.is_empty() {
            mul_two_slices(
                a1.as_ptr(),
                b1.as_ptr(),
                z2.as_mut_ptr(),
                base,
                a1.len() as u64,
                b1.len() as u64,
            );
        }

        let mut a01 = vec![0; a1.len() + 1];
        let mut b01 = vec![0; b1.len() + 1];
        let mut z1 = vec![0; a01.len() + b01.len() + 1];

        add_two_slices(
            a0.as_ptr(),
            a1.as_ptr(),
            a01.as_mut_ptr(),
            base,
            a1.len() as u64,
            a0.len() as u64,
        );
        add_two_slices(
            b0.as_ptr(),
            b1.as_ptr(),
            b01.as_mut_ptr(),
            base,
            b1.len() as u64,
            b0.len() as u64,
        );

        mul_two_slices(
            a01.as_ptr(),
            b01.as_ptr(),
            z1.as_mut_ptr(),
            base,
            a01.len() as u64,
            b01.len() as u64,
        );
        sub_two_slices(
            z1.as_ptr(),
            z2.as_ptr(),
            z1.as_mut_ptr(),
            base,
            z1.len() as u64,
            z2.len() as u64,
        );
        sub_two_slices(
            z1.as_ptr(),
            z0.as_ptr(),
            z1.as_mut_ptr(),
            base,
            z1.len() as u64,
            z0.len() as u64,
        );

        add_two_slices(
            dest.as_ptr(),
            z0.as_ptr(),
            dest.as_mut_ptr(),
            base,
            m as u64,
            z0.len() as u64,
        );
        add_two_slices(
            dest.as_ptr().offset(l as isize),
            z1.as_ptr(),
            dest.as_mut_ptr().offset(l as isize),
            base,
            m as u64,
            z1.len() as u64,
        );
        add_two_slices(
            dest.as_ptr().offset(2 * l as isize),
            z2.as_ptr(),
            dest.as_mut_ptr().offset(2 * l as isize),
            base,
            m as u64,
            z2.len() as u64,
        );
    }

    trim_zeros(&mut dest);
    dest
}

#[cfg(test)]
mod tests {
    use super::*;
}
