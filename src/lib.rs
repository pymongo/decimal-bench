//! Decimal benchmark

#[test]
fn decimal_rs_rescale() {
    let s = "123456.78901234000000";
    let y: decimal_rs::Decimal = s.parse().unwrap();
    dbg!(y);
    let y = y.normalize();
    dbg!(y);
    let r:rust_decimal::Decimal = s.parse().unwrap();
    println!("{}", s.parse::<f64>().unwrap());
    println!("{}", y);
    println!("{}", r);
}

#[test]
fn rescale() {
    let s = "123456.78901234000000";
    let y: pgnumeric::NumericBuf = s.parse().unwrap();
    println!("{}", y.trunc(5));
}

#[test]
fn rescale2() {
    let s = "123456.78901234000000";
    let y: bigdecimal::BigDecimal = s.parse().unwrap();
    println!("{}", y.with_scale(5));
}
