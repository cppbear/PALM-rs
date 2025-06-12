// Answer 0

#[test]
fn test_read_varu32_case_1() {
    let data: &[u8] = &[128, 128, 128, 128, 128];
    read_varu32(data);
}

#[test]
fn test_read_varu32_case_2() {
    let data: &[u8] = &[128];
    read_varu32(data);
}

#[test]
fn test_read_varu32_case_3() {
    let data: &[u8] = &[255];
    read_varu32(data);
}

#[test]
fn test_read_varu32_case_4() {
    let data: &[u8] = &[128, 0];
    read_varu32(data);
}

#[test]
fn test_read_varu32_case_5() {
    let data: &[u8] = &[255, 255, 255];
    read_varu32(data);
}

#[test]
fn test_read_varu32_case_6() {
    let data: &[u8] = &[128, 64, 32];
    read_varu32(data);
}

