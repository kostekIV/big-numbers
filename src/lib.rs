extern crate num_traits;

pub mod bint;

mod base_ops;
mod conversions;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
