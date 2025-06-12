// Answer 0

#[test]
fn test_remaining_empty_chain() {
    struct EmptyBuf;
    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize { 0 }
        fn chunk(&self) -> &[u8] { &[] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let chain = Chain { a: EmptyBuf, b: EmptyBuf };
    assert_eq!(chain.remaining(), 0);
}

#[test]
fn test_remaining_first_buf_non_empty() {
    struct NonEmptyBufA;
    impl Buf for NonEmptyBufA {
        fn remaining(&self) -> usize { 5 }
        fn chunk(&self) -> &[u8] { &[1, 2, 3, 4, 5] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    struct EmptyBuf;
    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize { 0 }
        fn chunk(&self) -> &[u8] { &[] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let chain = Chain { a: NonEmptyBufA, b: EmptyBuf };
    assert_eq!(chain.remaining(), 5);
}

#[test]
fn test_remaining_second_buf_non_empty() {
    struct EmptyBuf;
    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize { 0 }
        fn chunk(&self) -> &[u8] { &[] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    struct NonEmptyBufB;
    impl Buf for NonEmptyBufB {
        fn remaining(&self) -> usize { 7 }
        fn chunk(&self) -> &[u8] { &[1, 2, 3, 4, 5, 6, 7] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let chain = Chain { a: EmptyBuf, b: NonEmptyBufB };
    assert_eq!(chain.remaining(), 7);
}

#[test]
fn test_remaining_combined_bufs() {
    struct NonEmptyBufA;
    impl Buf for NonEmptyBufA {
        fn remaining(&self) -> usize { 5 }
        fn chunk(&self) -> &[u8] { &[1, 2, 3, 4, 5] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    struct NonEmptyBufB;
    impl Buf for NonEmptyBufB {
        fn remaining(&self) -> usize { 3 }
        fn chunk(&self) -> &[u8] { &[6, 7, 8] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let chain = Chain { a: NonEmptyBufA, b: NonEmptyBufB };
    assert_eq!(chain.remaining(), 8);
}

#[test]
fn test_remaining_only_first_buf_empty() {
    struct EmptyBuf;
    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize { 0 }
        fn chunk(&self) -> &[u8] { &[] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    struct NonEmptyBufC;
    impl Buf for NonEmptyBufC {
        fn remaining(&self) -> usize { 10 }
        fn chunk(&self) -> &[u8] { &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
        fn advance(&mut self, _: usize) {}
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let chain = Chain { a: EmptyBuf, b: NonEmptyBufC };
    assert_eq!(chain.remaining(), 10);
}

