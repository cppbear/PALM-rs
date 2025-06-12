// Answer 0

#[test]
fn test_write_vari32_zero() {
    let mut data = Vec::new();
    write_vari32(&mut data, 0);
}

#[test]
fn test_write_vari32_one() {
    let mut data = Vec::new();
    write_vari32(&mut data, 1);
}

#[test]
fn test_write_vari32_two() {
    let mut data = Vec::new();
    write_vari32(&mut data, 2);
}

#[test]
fn test_write_vari32_max() {
    let mut data = Vec::new();
    write_vari32(&mut data, 2147483647);
}

