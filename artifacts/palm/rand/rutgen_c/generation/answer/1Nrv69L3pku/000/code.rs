// Answer 0

#[test]
fn test_sample_alphanumeric() {
    struct DummyRng {
        value: u32,
    }

    impl Rng for DummyRng {
        fn next_u32(&mut self) -> u32 {
            let result = self.value;
            self.value += 1; // Increment for the next call
            result
        }
    }

    let mut rng = DummyRng { value: 0 };
    let alphanumeric = Alphanumeric;

    let sample1 = alphanumeric.sample(&mut rng);
    assert!(b"A" <= sample1 && sample1 <= b"Z" || b"a" <= sample1 && sample1 <= b"z" || b"0" <= sample1 && sample1 <= b"9");

    let mut rng2 = DummyRng { value: 63 };
    let sample2 = alphanumeric.sample(&mut rng2);
    assert!(sample2 == b"z");
    
    let mut rng3 = DummyRng { value: 62 };
    let sample3 = alphanumeric.sample(&mut rng3);
    assert!(sample3 == b"9");
}

