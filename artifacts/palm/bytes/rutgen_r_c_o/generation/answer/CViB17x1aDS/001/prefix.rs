// Answer 0

#[test]
fn test_split_off_must_use_case_1() {
    let mut b1 = Bytes::from("hello");
    b1.split_off(0);
}

#[test]
fn test_split_off_must_use_case_2() {
    let mut b1 = Bytes::from("hello");
    b1.split_off(5);
}

#[test]
fn test_split_off_must_use_case_3() {
    let mut b1 = Bytes::from("hello");
    b1.split_off(3);
}

#[test]
fn test_split_off_must_use_case_4() {
    let mut b1 = Bytes::from("world");
    b1.split_off(0);
}

#[test]
fn test_split_off_must_use_case_5() {
    let mut b1 = Bytes::from("world");
    b1.split_off(5);
}

#[test]
fn test_split_off_must_use_case_6() {
    let mut b1 = Bytes::from("world");
    b1.split_off(2);
}

#[test]
fn test_split_off_must_use_case_7() {
    let mut b1 = Bytes::from("");
    b1.split_off(0);
}

#[test]
#[should_panic]
fn test_split_off_must_use_case_8() {
    let mut b1 = Bytes::from("test");
    b1.split_off(5);
}

#[test]
#[should_panic]
fn test_split_off_must_use_case_9() {
    let mut b1 = Bytes::from("test");
    b1.split_off(6);
}

