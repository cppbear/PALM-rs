// Answer 0

#[test]
fn test_read_varu32_case1() {
    let data: &[u8] = &[0b0000_0001]; 
    let result = read_varu32(data);
    assert_eq!(result, (1, 1));
}

#[test]
fn test_read_varu32_case2() {
    let data: &[u8] = &[0b1000_0000]; 
    let result = read_varu32(data);
    assert_eq!(result, (0, 1));
}

#[test]
fn test_read_varu32_case3() {
    let data: &[u8] = &[0b1111_1111, 0b0000_0001]; 
    let result = read_varu32(data);
    assert_eq!(result, (127, 1));
}

#[test]
fn test_read_varu32_case4() {
    let data: &[u8] = &[0b1111_1111, 0b1111_1111]; 
    let result = read_varu32(data);
    assert_eq!(result, (16383, 2));
}

#[test]
fn test_read_varu32_case5() {
    let data: &[u8] = &[0b1000_0000, 0b0000_0000]; 
    let result = read_varu32(data);
    assert_eq!(result, (0, 1));
}

#[test]
fn test_read_varu32_case6() {
    let data: &[u8] = &[0b1111_1111, 0b1111_1111, 0b0000_0001]; 
    let result = read_varu32(data);
    assert_eq!(result, (2097151, 3));
}

#[test]
fn test_read_varu32_case7() {
    let data: &[u8] = &[]; 
    let result = read_varu32(data);
    assert_eq!(result, (0, 0));
}

