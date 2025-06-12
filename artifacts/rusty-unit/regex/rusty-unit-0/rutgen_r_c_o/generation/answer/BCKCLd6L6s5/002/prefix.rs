// Answer 0

#[test]
fn test_read_vari32_case1() {
    let data = [0b0000_0000]; // Input for un = 0 (expect (0, 1))
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case2() {
    let data = [0b0000_0001]; // Input for un = 1 (expect (0, 1))
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case3() {
    let data = [0b0000_0000, 0b0000_0001]; // Input for un = 0 (expect (0, 1))
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case4() {
    let data = [0b0000_0001, 0b0000_0000, 0b0000_0001]; // Input for un = 1 (expect (0, 1))
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case5() {
    let data = [0b1000_0000, 0b0000_0000]; // Input for un = 128 (expect (64, 2))
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case6() {
    let data = [0b0111_1111, 0b0000_0001]; // Input for un = 127 (expect (63, 2))
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case7() {
    let data = [0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0111_1111]; // Input for un = 2147483647 (expect (1073741823, 4))
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case8() {
    let data = [0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0111_1110]; // Input for un = 2147483646 (expect (1073741823, 4))
    read_vari32(&data);
}

