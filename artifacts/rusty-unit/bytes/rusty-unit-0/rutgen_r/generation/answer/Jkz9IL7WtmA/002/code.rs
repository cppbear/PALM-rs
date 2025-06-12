// Answer 0

#[test]
#[should_panic(expected = "assertion failed")]
fn test_advance_panic_case() {
    struct Inner {
        count: usize,
    }

    impl Inner {
        fn advance(&mut self, cnt: usize) {
            self.count += cnt;
        }
    }

    struct Buffer {
        inner: Inner,
        limit: usize,
    }

    let mut buffer = Buffer {
        inner: Inner { count: 0 },
        limit: 5,
    };

    // This should panic since cnt (10) is greater than limit (5)
    buffer.advance(10);
}

