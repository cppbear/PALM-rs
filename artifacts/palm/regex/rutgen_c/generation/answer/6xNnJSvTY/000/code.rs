// Answer 0

#[test]
fn test_should_exec_with_zero_insts_and_zero_text_len() {
    assert_eq!(should_exec(0, 0), true);
}

#[test]
fn test_should_exec_with_max_insts_and_min_text_len() {
    let num_insts = (MAX_SIZE_BYTES / 4) * BIT_SIZE / 1 - 1; // max insts such that size is <= MAX_SIZE_BYTES
    assert_eq!(should_exec(num_insts, 0), true);
}

#[test]
fn test_should_exec_with_zero_insts_and_max_text_len() {
    assert_eq!(should_exec(0, MAX_SIZE_BYTES), true);
}

#[test]
fn test_should_exec_with_min_insts_and_max_text_len() {
    assert_eq!(should_exec(1, MAX_SIZE_BYTES), false);
}

#[test]
fn test_should_exec_with_high_insts_and_high_text_len() {
    let num_insts = (MAX_SIZE_BYTES / 4) * BIT_SIZE / (MAX_SIZE_BYTES + 1); // adjust to ensure size exceeds limit
    assert_eq!(should_exec(num_insts, MAX_SIZE_BYTES), false);
}

