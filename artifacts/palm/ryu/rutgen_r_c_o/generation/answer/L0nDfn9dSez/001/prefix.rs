// Answer 0

#[test]
fn test_multiple_of_power_of_2_case1() {
    let value = 1;
    let p = 0;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case2() {
    let value = 2;
    let p = 1;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case3() {
    let value = 4;
    let p = 2;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case4() {
    let value = 8;
    let p = 3;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case5() {
    let value = 16;
    let p = 4;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case6() {
    let value = 32;
    let p = 5;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case7() {
    let value = 64;
    let p = 6;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case8() {
    let value = 128;
    let p = 7;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case9() {
    let value = 255;
    let p = 8;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case10() {
    let value = 1024;
    let p = 10;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case11() {
    let value = 4096;
    let p = 12;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case12() {
    let value = 65536;
    let p = 16;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case13() {
    let value = 1048576;
    let p = 20;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case14() {
    let value = 4294967295;
    let p = 32;
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_case15() {
    let value = 18446744073709551615;
    let p = 63;
    multiple_of_power_of_2(value, p);
}

