// Answer 0

#[test]
fn test_should_use_minimum_valid_length() {
    let pattern = vec![150, 151, 152, 153, 154, 155, 156, 157, 158, 159];
    should_use(&pattern);
}

#[test]
fn test_should_use_exceeding_minimum_length_all_above_cutoff() {
    let pattern = vec![160, 161, 162, 163, 164, 165, 166, 167, 168, 169];
    should_use(&pattern);
}

#[test]
fn test_should_use_edge_case_maximum_length() {
    let pattern = vec![200; 256]; // All characters are the same, but length is max
    should_use(&pattern);
}

#[test]
fn test_should_use_mixed_characters_above_cutoff() {
    let pattern = vec![150, 155, 160, 165, 170, 175, 180, 185, 190, 195];
    should_use(&pattern);
}

#[test]
fn test_should_use_high_frequency_characters() {
    let pattern = vec![200, 201, 202, 203, 204, 205, 206, 207, 208, 209];
    should_use(&pattern);
}

#[test]
#[should_panic]
fn test_should_use_below_minimum_length() {
    let pattern = vec![150, 151, 152, 153, 154, 155, 156, 157, 158]; // Length <= 9
    should_use(&pattern);
}

#[test]
#[should_panic]
fn test_should_use_contains_low_frequency_characters() {
    let pattern = vec![149, 150, 151, 152, 153, 154, 155, 156, 157, 158]; // Contains a rank below cutoff
    should_use(&pattern);
}

