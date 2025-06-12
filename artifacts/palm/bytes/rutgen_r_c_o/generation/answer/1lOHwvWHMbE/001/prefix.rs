// Answer 0

#[test]
fn test_sign_extend_with_min_val_and_min_nbytes() {
    let val = 0;
    let nbytes = 1;
    sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_with_zero_val_and_max_nbytes() {
    let val = 0;
    let nbytes = 8;
    sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_with_max_val_and_min_nbytes() {
    let val = u64::MAX;
    let nbytes = 1;
    sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_with_max_val_and_max_nbytes() {
    let val = u64::MAX;
    let nbytes = 8;
    sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_with_mid_val_and_nbytes_4() {
    let val = 0b1111_1111_1111_1111_1111_1111_1111_1111;
    let nbytes = 4;
    sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_with_non_max_val_and_nbytes_7() {
    let val = 0b0111_1111_1111_1111_1111_1111_1111_1111;
    let nbytes = 7;
    sign_extend(val, nbytes);
}

