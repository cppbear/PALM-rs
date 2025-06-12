// Answer 0

#[test]
fn test_fold_impl() {
    struct TestBucket {
        value: i32,
    }

    struct TestRawIterRange {
        current_group: BitMaskIter,
        data: Bucket<TestBucket>,
        next_ctrl: *const u8,
        end: *const u8,
    }

    impl TestRawIterRange {
        unsafe fn new(ctrl: *const u8, data: Bucket<TestBucket>, len: usize) -> Self {
            TestRawIterRange {
                current_group: BitMaskIter(BitMask(0)),
                data,
                next_ctrl: ctrl.add(1),
                end: ctrl.add(len),
            }
        }

        unsafe fn fold_impl<F, B>(&mut self, mut n: usize, mut acc: B, mut f: F) -> B
        where
            F: FnMut(B, Bucket<TestBucket>) -> B,
        {
            loop {
                while let Some(index) = self.current_group.next() {
                    assert!(n != 0);
                    let bucket = self.data.next_n(index);
                    acc = f(acc, bucket);
                    n -= 1;
                }
                if n == 0 {
                    return acc;
                }
                self.current_group = BitMaskIter(BitMask(0));
                self.data = self.data.next_n(1);
                self.next_ctrl = self.next_ctrl.add(1);
            }
        }
    }

    unsafe fn test_fold() {
        let bucket_array = Box::new([
            TestBucket { value: 1 },
            TestBucket { value: 2 },
            TestBucket { value: 3 },
        ]);
        let ptr = NonNull::new(Box::into_raw(bucket_array)).unwrap();
        let bucket = Bucket { ptr };

        let ctrl = ptr.as_ptr() as *const u8;
        let mut range = TestRawIterRange::new(ctrl, bucket, 3);

        let result = range.fold_impl(3, 0, |acc, bucket| acc + bucket.value);
        assert_eq!(result, 6);
    }
    
    unsafe {
        test_fold();
    }
}

#[test]
#[should_panic]
fn test_fold_impl_empty() {
    struct TestBucket {
        value: i32,
    }

    struct TestRawIterRange {
        current_group: BitMaskIter,
        data: Bucket<TestBucket>,
        next_ctrl: *const u8,
        end: *const u8,
    }

    impl TestRawIterRange {
        unsafe fn new(ctrl: *const u8, data: Bucket<TestBucket>, len: usize) -> Self {
            TestRawIterRange {
                current_group: BitMaskIter(BitMask(0)),
                data,
                next_ctrl: ctrl.add(1),
                end: ctrl.add(len),
            }
        }

        unsafe fn fold_impl<F, B>(&mut self, mut n: usize, mut acc: B, mut f: F) -> B
        where
            F: FnMut(B, Bucket<TestBucket>) -> B,
        {
            loop {
                while let Some(index) = self.current_group.next() {
                    assert!(n != 0);
                    let bucket = self.data.next_n(index);
                    acc = f(acc, bucket);
                    n -= 1;
                }
                if n == 0 {
                    return acc;
                }
                self.current_group = BitMaskIter(BitMask(0));
                self.data = self.data.next_n(1);
                self.next_ctrl = self.next_ctrl.add(1);
            }
        }
    }

    unsafe fn test_empty_fold() {
        let bucket_array = Box::new([]);
        let ptr = NonNull::new(Box::into_raw(bucket_array)).unwrap();
        let bucket = Bucket { ptr };

        let ctrl = ptr.as_ptr() as *const u8;
        let mut range = TestRawIterRange::new(ctrl, bucket, 0);
        
        let _result = range.fold_impl(0, 0, |acc, _bucket| acc);
    }

    unsafe {
        test_empty_fold();
    }
}

