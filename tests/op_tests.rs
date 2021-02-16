use big_numbers::int::Int;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

macro_rules! expr_identity {
    ($e:expr) => {
        $e
    };
}

macro_rules! op_test {
    ($func:ident, $path:literal, $op:tt) => {
        #[test]
        #[ignore]
        fn $func() -> io::Result<()> {
            let file = File::open($path)?;
            let reader = BufReader::new(file);
            let base = u64::pow(2, 32);

            for line in reader.lines() {
                if let Ok(abr) = line {
                    let v: Vec<&str> = abr.split(',').collect();

                    let a = Int::from((10u64, base, v[0]));
                    let b = Int::from((10u64, base, v[1]));

                    assert_eq!(v[2], expr_identity!((&a $op &b).to_string()));
                }
            }
            Ok(())
        }
    }
}

op_test!(add_test, "./tests/add_test.csv", +);
op_test!(sub_test, "./tests/sub_test.csv", -);
op_test!(mul_test, "./tests/mul_test.csv", *);
