use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use big_numbers::bint::Bint;


#[test]
fn mul_test() -> io::Result<()> {
    let file = File::open("./tests/mul_test.csv")?;
    let reader = BufReader::new(file);
    let base = u64::pow(2, 32);

    for line in reader.lines() {
        if let Ok(abr) = line {
            let v: Vec<&str> = abr.split(',').collect();

            let a = Bint::from((10u64, base, v[0]));
            let b = Bint::from((10u64, base, v[1]));

            assert_eq!(v[2], (&a * &b).to_string());
        }
    }
    Ok(())
}

#[test]
fn add_test() -> io::Result<()> {
    let file = File::open("./tests/add_test.csv")?;
    let reader = BufReader::new(file);
    let base = u64::pow(2, 32);

    for line in reader.lines() {
        if let Ok(abr) = line {
            let v: Vec<&str> = abr.split(',').collect();

            let a = Bint::from((10u64, base, v[0]));
            let b = Bint::from((10u64, base, v[1]));

            assert_eq!(v[2], (&a + &b).to_string());
        }
    }
    Ok(())
}
#[test]
fn sub_test() -> io::Result<()> {
    let file = File::open("./tests/sub_test.csv")?;
    let reader = BufReader::new(file);
    let base = u64::pow(2, 32);

    for line in reader.lines() {
        if let Ok(abr) = line {
            let v: Vec<&str> = abr.split(',').collect();

            let a = Bint::from((10u64, base, v[0]));
            let b = Bint::from((10u64, base, v[1]));

            assert_eq!(v[2], (&a - &b).to_string());
        }
    }
    Ok(())
}
