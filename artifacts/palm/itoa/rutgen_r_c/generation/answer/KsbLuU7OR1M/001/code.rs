// Answer 0

#[test]
#[should_panic]
fn test_format_i8_exceeding_length() {
    let mut buffer = Buffer::new();
    let input: i8 = 127; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "127");

    let input: i8 = -128; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "-128");

    // Exceeding length condition
    let exceeding_input: i8 = 1000; // exceeding i8 max length of 4
    let _output = buffer.format(exceeding_input); // should panic
}

#[test]
#[should_panic]
fn test_format_i16_exceeding_length() {
    let mut buffer = Buffer::new();
    let input: i16 = 32767; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "32767");

    let input: i16 = -32768; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "-32768");

    // Exceeding length condition
    let exceeding_input: i16 = 100000; // exceeding i16 max length of 6
    let _output = buffer.format(exceeding_input); // should panic
}

#[test]
#[should_panic]
fn test_format_i32_exceeding_length() {
    let mut buffer = Buffer::new();
    let input: i32 = 2147483647; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "2147483647");

    let input: i32 = -2147483648; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "-2147483648");

    // Exceeding length condition
    let exceeding_input: i32 = 10000000000; // exceeding i32 max length of 11
    let _output = buffer.format(exceeding_input); // should panic
}

#[test]
#[should_panic]
fn test_format_i64_exceeding_length() {
    let mut buffer = Buffer::new();
    let input: i64 = 9223372036854775807; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "9223372036854775807");

    let input: i64 = -9223372036854775808; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "-9223372036854775808");

    // Exceeding length condition
    let exceeding_input: i64 = 10000000000000000000; // exceeding i64 max length of 20
    let _output = buffer.format(exceeding_input); // should panic
}

#[test]
#[should_panic]
fn test_format_i128_exceeding_length() {
    let mut buffer = Buffer::new();
    let input: i128 = 170141183460469231731687303715884105727; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "170141183460469231731687303715884105727");

    let input: i128 = -170141183460469231731687303715884105728; // should fit
    let output = buffer.format(input);
    assert_eq!(output, "-170141183460469231731687303715884105728");

    // Exceeding length condition
    let exceeding_input: i128 = 100000000000000000000000000000000000000; // exceeding i128 max length of 40
    let _output = buffer.format(exceeding_input); // should panic
}

