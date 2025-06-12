// Answer 0

#[test]
fn test_read_vari32_case_0() {
    let data = vec![0b0000_0001]; // Represents the value -1
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_1() {
    let data = vec![0b0000_0011]; // Represents the value -2
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_2() {
    let data = vec![0b0000_0101]; // Represents the value -3
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_3() {
    let data = vec![0b0000_0111]; // Represents the value -4
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_4() {
    let data = vec![0b0000_1001]; // Represents the value -5
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_5() {
    let data = vec![0b0000_1011]; // Represents the value -6
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_6() {
    let data = vec![0b0000_1101]; // Represents the value -7
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_7() {
    let data = vec![0b0000_1111]; // Represents the value -8
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_8() {
    let data = vec![0b0001_0001]; // Represents the value -9
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_9() {
    let data = vec![0b0001_0011]; // Represents the value -10
    read_vari32(&data);
}

#[test]
fn test_read_vari32_case_10() {
    let data = vec![0b0001_0101]; // Represents the value -11
    read_vari32(&data);
}

