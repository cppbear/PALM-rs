// Answer 0

#[test]
fn test_fmt_bool() {
    let val: bool = true;
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "true");
}

#[test]
fn test_fmt_string() {
    let val: String = "Hello".to_string();
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "Hello");
}

#[test]
fn test_fmt_char() {
    let val: char = 'A';
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "A");
}

#[test]
fn test_fmt_u8() {
    let val: u8 = 255;
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "255");
}

#[test]
fn test_fmt_u16() {
    let val: u16 = 65535;
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "65535");
}

#[test]
fn test_fmt_str() {
    let val: &str = "Test string";
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "Test string");
}

#[test]
fn test_fmt_u32() {
    let val: u32 = 4294967295;
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "4294967295");
}

#[test]
fn test_fmt_u64() {
    let val: u64 = 18446744073709551615;
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "18446744073709551615");
}

#[test]
fn test_fmt_u128() {
    let val: u128 = 340282366920938463463374607431768211455;
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, "340282366920938463463374607431768211455");
}

#[test]
fn test_fmt_usize() {
    let val: usize = usize::MAX;
    let result = std::fmt::Formatter::new();
    let mut output = String::new();
    {
        let mut formatter = &mut output;
        let _ = val.fmt(&mut formatter);
    }
    assert_eq!(output, format!("{}", usize::MAX));
}

