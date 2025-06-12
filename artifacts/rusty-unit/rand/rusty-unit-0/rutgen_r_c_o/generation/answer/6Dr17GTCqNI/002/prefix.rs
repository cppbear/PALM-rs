// Answer 0

#[derive(Copy, Clone)]
struct TestData {
    value: u32,
}

impl Observable for TestData {
    type Bytes = [u8; 4];

    fn to_le_bytes(self) -> Self::Bytes {
        self.value.to_le_bytes()
    }
}

#[test]
fn test_fill_via_chunks_case_1() {
    let src = [TestData { value: 1 }];
    let mut dest = [0u8; 8];
    let result = fill_via_chunks(&src, &mut dest);
}

#[test]
fn test_fill_via_chunks_case_2() {
    let src = [TestData { value: 1 }, TestData { value: 2 }];
    let mut dest = [0u8; 12];
    let result = fill_via_chunks(&src, &mut dest);
}

#[test]
fn test_fill_via_chunks_case_3() {
    let src = [TestData { value: 1 }, TestData { value: 2 }, TestData { value: 3 }];
    let mut dest = [0u8; 16];
    let result = fill_via_chunks(&src, &mut dest);
}

#[test]
fn test_fill_via_chunks_case_with_remainder() {
    let src = [TestData { value: 1 }, TestData { value: 2 }];
    let mut dest = [0u8; 10];
    let result = fill_via_chunks(&src, &mut dest);
}

#[test]
fn test_fill_via_chunks_case_exact_size() {
    let src = [TestData { value: 1 }];
    let mut dest = [0u8; 4];
    let result = fill_via_chunks(&src, &mut dest);
}

