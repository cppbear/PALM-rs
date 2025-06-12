// Answer 0

#[test]
fn test_fill_via_chunks_empty_dest() {
    #[derive(Copy, Clone)]
    struct TestType(u32);
    
    impl Observable for TestType {
        type Bytes = [u8; 4];
        fn to_le_bytes(self) -> Self::Bytes {
            self.0.to_le_bytes()
        }
    }
    
    let src: &[TestType] = &[TestType(1)];
    let mut dest: [u8; 0] = [];
    let result = fill_via_chunks(src, &mut dest);
}

#[test]
fn test_fill_via_chunks_one_element_src() {
    #[derive(Copy, Clone)]
    struct TestType(u32);
    
    impl Observable for TestType {
        type Bytes = [u8; 4];
        fn to_le_bytes(self) -> Self::Bytes {
            self.0.to_le_bytes()
        }
    }
    
    let src: &[TestType] = &[TestType(2)];
    let mut dest: [u8; 0] = [];
    let result = fill_via_chunks(src, &mut dest);
}

