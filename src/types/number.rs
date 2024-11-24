pub fn number_type() -> () {
    let y: i64 = 92_000_000i64;
    println!("y: {}", y);

    let hex_octal_bin : i64 = 0xffff_ffff + 0o777 + 0b1;
    println!("hex_octal_bin: {}", hex_octal_bin);
    let byte: u8 = b'a';
    println!("byte: {}", byte);
    assert_eq!(byte, 97);
    println!("{} - {}", 8.5f32.ceil().sin().round().sqrt(), 90f64.sin());

    let f64_nan = f64::NAN;
    println!("f64::NAN: {}", f64_nan);
}