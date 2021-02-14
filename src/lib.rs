pub mod int;

mod base_ops;
mod conversions;
mod asm_ops;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
