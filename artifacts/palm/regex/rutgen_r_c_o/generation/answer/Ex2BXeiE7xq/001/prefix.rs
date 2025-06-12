// Answer 0

#[test]
fn test_new_valid_range() {
    let range = new(0, 255);
    let range2 = new(100, 200);
}

#[test]
fn test_new_equal_start_end() {
    let range = new(128, 128);
    let range2 = new(0, 0);
}

#[test]
fn test_new_start_less_than_end() {
    let range = new(1, 2);
    let range3 = new(10, 20);
}

#[test]
#[should_panic]
fn test_new_start_greater_than_end() {
    let range = new(200, 100);
}

#[test]
fn test_new_bounds() {
    let range1 = new(0, 0);
    let range2 = new(255, 255);
    let range3 = new(255, 0); // this will panic
}

#[test]
fn test_new_mid_range() {
    let range = new(127, 200);
    let range2 = new(50, 150);
}

