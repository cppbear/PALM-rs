// Answer 0

#[test]
fn test_should_use_minimum_length() {
    let pattern: Vec<u8> = vec![150; 9]; // length 9, all bytes have freq_rank >= 150
    should_use(pattern.as_slice());
}

#[test]
fn test_should_use_just_above_min_length() {
    let pattern: Vec<u8> = vec![151; 10]; // length 10, all bytes have freq_rank >= 150
    should_use(pattern.as_slice());
}

#[test]
fn test_should_use_exactly_min_cutoff() {
    let pattern: Vec<u8> = vec![150; 9]; // length 9, just at the cutoff frequency
    should_use(pattern.as_slice());
}

#[test]
fn test_should_use_all_characters_above_cutoff() {
    let pattern: Vec<u8> = vec![160, 161, 162, 163, 164, 165, 166, 167, 168]; // length 9, all bytes with freq_rank >= 150
    should_use(pattern.as_slice());
}

#[test]
fn test_should_use_with_mixed_characters_above_cutoff() {
    let pattern: Vec<u8> = vec![150, 155, 160, 165, 170, 172, 180, 190, 200]; // length 9, all bytes with freq_rank >= 150
    should_use(pattern.as_slice());
}

