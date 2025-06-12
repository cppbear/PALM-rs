// Answer 0

#[cfg(test)]
fn test_random_u8() {
    let value: u8 = rand::random();
    assert!(value <= 255);
}

#[cfg(test)]
fn test_random_f64() {
    let value: f64 = rand::random();
    assert!(value >= 0.0 && value <= 1.0);
}

#[cfg(test)]
fn test_random_bool() {
    let value: bool = rand::random();
    assert!(value == true || value == false);
}

