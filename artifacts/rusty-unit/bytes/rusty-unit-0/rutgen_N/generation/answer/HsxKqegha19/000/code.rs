// Answer 0

#[test]
fn test_get_mut() {
    struct TestBufMut {
        capacity: usize,
    }
    
    impl TestBufMut {
        fn reserve(&mut self, additional: usize) {
            self.capacity += additional;
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    struct TestWriter {
        buf: TestBufMut,
    }

    impl TestWriter {
        fn get_mut(&mut self) -> &mut TestBufMut {
            &mut self.buf
        }

        fn get_ref(&self) -> &TestBufMut {
            &self.buf
        }
    }

    let mut writer = TestWriter {
        buf: TestBufMut { capacity: 0 },
    };

    writer.get_mut().reserve(1024);
    
    assert_eq!(1024, writer.get_ref().capacity());
}

