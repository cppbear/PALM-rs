// Answer 0


#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    struct Inner {
        limit: usize,
        remaining_value: usize,
    }

    impl Inner {
        fn remaining(&self) -> usize {
            self.remaining_value
        }
    }

    struct TestStruct {
        inner: Inner,
        limit: usize,
    }

    impl TestStruct {
        fn remaining(&self) -> usize {
            cmp::min(self.inner.remaining(), self.limit)
        }
    }

    #[test]
    fn test_remaining_with_limit_greater_than_remaining() {
        let inner = Inner {
            limit: 5,
            remaining_value: 10,
        };
        let test_struct = TestStruct {
            inner,
            limit: 6,
        };
        assert_eq!(test_struct.remaining(), 6);
    }

    #[test]
    fn test_remaining_with_limit_less_than_remaining() {
        let inner = Inner {
            limit: 5,
            remaining_value: 10,
        };
        let test_struct = TestStruct {
            inner,
            limit: 5,
        };
        assert_eq!(test_struct.remaining(), 5);
    }

    #[test]
    fn test_remaining_with_limit_equal_to_remaining() {
        let inner = Inner {
            limit: 5,
            remaining_value: 5,
        };
        let test_struct = TestStruct {
            inner,
            limit: 5,
        };
        assert_eq!(test_struct.remaining(), 5);
    }

    #[test]
    fn test_remaining_with_zero_limit() {
        let inner = Inner {
            limit: 5,
            remaining_value: 10,
        };
        let test_struct = TestStruct {
            inner,
            limit: 0,
        };
        assert_eq!(test_struct.remaining(), 0);
    }

    #[test]
    fn test_remaining_with_zero_remaining() {
        let inner = Inner {
            limit: 5,
            remaining_value: 0,
        };
        let test_struct = TestStruct {
            inner,
            limit: 10,
        };
        assert_eq!(test_struct.remaining(), 0);
    }
}


