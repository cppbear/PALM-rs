// Answer 0

#[test]
fn test_add_padding_len_0() {
    let mut output = [0u8; 4];
    let result = add_padding(0, &mut output);
}

#[test]
fn test_add_padding_len_1() {
    let mut output = [0u8; 4];
    let result = add_padding(1, &mut output);
}

#[test]
fn test_add_padding_len_2() {
    let mut output = [0u8; 4];
    let result = add_padding(2, &mut output);
}

#[test]
fn test_add_padding_len_3() {
    let mut output = [0u8; 4];
    let result = add_padding(3, &mut output);
}

#[test]
fn test_add_padding_len_0_min_output() {
    let mut output = [0u8; 2];
    let result = add_padding(0, &mut output);
}

#[test]
fn test_add_padding_len_1_min_output() {
    let mut output = [0u8; 2];
    let result = add_padding(1, &mut output);
}

#[test]
fn test_add_padding_len_2_min_output() {
    let mut output = [0u8; 2];
    let result = add_padding(2, &mut output);
}

#[test]
fn test_add_padding_len_3_min_output() {
    let mut output = [0u8; 2];
    let result = add_padding(3, &mut output);
}

