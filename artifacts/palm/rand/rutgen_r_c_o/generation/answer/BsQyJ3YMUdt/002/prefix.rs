// Answer 0

#[test]
fn test_read_u32le_valid_input_1() {
    let xs = [0, 1, 2, 3];
    let _ = read_u32le(&xs);
}

#[test]
fn test_read_u32le_valid_input_2() {
    let xs = [255, 254, 253, 252];
    let _ = read_u32le(&xs);
}

#[test]
fn test_read_u32le_valid_input_3() {
    let xs = [0, 0, 0, 0];
    let _ = read_u32le(&xs);
}

#[test]
fn test_read_u32le_valid_input_4() {
    let xs = [0, 255, 255, 255];
    let _ = read_u32le(&xs);
}

#[should_panic]
#[test]
fn test_read_u32le_invalid_length() {
    let xs = [0, 1, 2];
    let _ = read_u32le(&xs);
}

#[should_panic]
#[test]
fn test_read_u32le_invalid_length_extra() {
    let xs = [0, 1, 2, 3, 4];
    let _ = read_u32le(&xs);
}

