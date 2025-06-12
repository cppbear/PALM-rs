// Answer 0

#[test]
fn test_fill_bytes_with_empty_array() {
    struct Xoshiro256PlusPlus;

    impl Xoshiro256PlusPlus {
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            fill_bytes_via_next(self, dst);
        }
    }

    let mut rng = Xoshiro256PlusPlus;
    let mut buf: [u8; 0] = [];
    rng.fill_bytes(&mut buf);
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_fill_bytes_with_small_array() {
    struct Xoshiro256PlusPlus;

    impl Xoshiro256PlusPlus {
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            fill_bytes_via_next(self, dst);
        }
    }

    let mut rng = Xoshiro256PlusPlus;
    let mut buf = [0u8; 4];
    rng.fill_bytes(&mut buf);
    assert_ne!(buf, [0, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_with_large_array() {
    struct Xoshiro256PlusPlus;

    impl Xoshiro256PlusPlus {
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            fill_bytes_via_next(self, dst);
        }
    }

    let mut rng = Xoshiro256PlusPlus;
    let mut buf = [0u8; 1024];
    rng.fill_bytes(&mut buf);
    assert_ne!(buf, [0; 1024]);
}

