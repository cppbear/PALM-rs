// Answer 0

#[derive(Debug)]
struct TestStructA;
#[derive(Debug)]
struct TestStructB;

struct SplitStruct;

impl SplitStruct {
    fn split(self) -> (TestStructA, TestStructB) {
        (TestStructA, TestStructB)
    }
}

#[test]
fn test_split() {
    let split_struct = SplitStruct;
    let (a, b) = split_struct.split();
    assert!(std::mem::size_of_val(&a) > 0);
    assert!(std::mem::size_of_val(&b) > 0);
}

