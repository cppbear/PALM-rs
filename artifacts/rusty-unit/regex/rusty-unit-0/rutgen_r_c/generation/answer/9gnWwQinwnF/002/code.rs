// Answer 0

#[test]
fn test_write_vari32_zero() {
    let mut data = Vec::new();
    let n = 0;
    write_vari32(&mut data, n);
    assert_eq!(data, vec![0]);
}

#[test]
fn test_write_vari32_positive() {
    let mut data = Vec::new();
    let n = 1;
    write_vari32(&mut data, n);
    assert_eq!(data, vec![2]);
}

#[test]
fn test_write_vari32_large_positive() {
    let mut data = Vec::new();
    let n = 127;
    write_vari32(&mut data, n);
    assert_eq!(data, vec![254]);
}

#[test]
fn test_write_vari32_negative() {
    let mut data = Vec::new();
    let n = -1;
    write_vari32(&mut data, n);
    assert_eq!(data, vec![1]);
}

#[test]
fn test_write_vari32_boundary_negative() {
    let mut data = Vec::new();
    let n = -128;
    write_vari32(&mut data, n);
    assert_eq!(data, vec![255, 1]);
}

