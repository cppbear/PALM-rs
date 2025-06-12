// Answer 0

#[test]
fn test_write_str_overflow_case_1() {
    let mut buffer = [0u8; 10];
    let offset = 10;
    let mut buf = Buf { bytes: &mut buffer, offset };
    let input_str = "test";
    let _ = buf.write_str(input_str);
}

#[test]
fn test_write_str_overflow_case_2() {
    let mut buffer = [0u8; 15];
    let offset = 10;
    let mut buf = Buf { bytes: &mut buffer, offset };
    let input_str = "longer than capacity";
    let _ = buf.write_str(input_str);
}

#[test]
fn test_write_str_overflow_case_3() {
    let mut buffer = [0u8; 20];
    let offset = 15;
    let mut buf = Buf { bytes: &mut buffer, offset };
    let input_str = "more than five";
    let _ = buf.write_str(input_str);
}

#[test]
fn test_write_str_overflow_case_4() {
    let mut buffer = [0u8; 10];
    let offset = 5;
    let mut buf = Buf { bytes: &mut buffer, offset };
    let input_str = "hello";
    let _ = buf.write_str(input_str);
}

#[test]
fn test_write_str_overflow_case_5() {
    let mut buffer = [0u8; 12];
    let offset = 11;
    let mut buf = Buf { bytes: &mut buffer, offset };
    let input_str = "exceeding";
    let _ = buf.write_str(input_str);
}

