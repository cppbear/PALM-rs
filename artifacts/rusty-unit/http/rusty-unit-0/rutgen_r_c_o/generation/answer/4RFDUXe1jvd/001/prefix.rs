// Answer 0

#[test]
fn test_write_with_zero_length_input() {
    let mut hasher = IdHasher::default();
    let input: &[u8] = &[0; 1];
    hasher.write(input);
}

#[test]
fn test_write_with_eight_length_input() {
    let mut hasher = IdHasher::default();
    let input: &[u8] = &[0; 8];
    hasher.write(input);
}

#[test]
fn test_write_with_thirty_two_length_input() {
    let mut hasher = IdHasher::default();
    let input: &[u8] = &[1; 32];
    hasher.write(input);
}

#[test]
fn test_write_with_sixty_four_length_input() {
    let mut hasher = IdHasher::default();
    let input: &[u8] = &[1; 64];
    hasher.write(input);
}

