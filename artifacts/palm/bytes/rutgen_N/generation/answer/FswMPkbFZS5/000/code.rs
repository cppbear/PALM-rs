// Answer 0

#[test]
fn test_assert_trait_object() {
    struct MyBuf {
        data: Vec<u8>,
    }

    impl MyBuf {
        fn new(data: Vec<u8>) -> Self {
            MyBuf { data }
        }
    }
    
    use std::ops::Deref;

    trait Buf {
        fn as_slice(&self) -> &[u8];
    }

    impl Buf for MyBuf {
        fn as_slice(&self) -> &[u8] {
            self.data.as_slice()
        }
    }

    let my_buf = MyBuf::new(vec![1, 2, 3, 4]);
    _assert_trait_object(&my_buf);
}

