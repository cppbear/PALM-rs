// Answer 0

#[test]
fn test_sample_valid_range() {
    struct MockRng {
        value: usize,
    }

    impl crate::Rng for MockRng {
        fn gen_range(&mut self, start: usize, end: usize) -> usize {
            self.value % (end - start) + start
        }
    }

    let slice = &[10, 20, 30, 40, 50];
    let range = Uniform::new(0, slice.len());
    let mut rng = MockRng { value: 2 }; // When this is called, it should produce index 2

    let result = sample(slice, &mut rng);
    assert_eq!(*result, 30);
}

#[test]
fn test_sample_out_of_bounds() {
    struct MockRng {
        value: usize,
    }

    impl crate::Rng for MockRng {
        fn gen_range(&mut self, start: usize, end: usize) -> usize {
            // This will create an out of bounds situation.
            10 // Invalid index for a slice of length 5
        }
    }

    let slice = &[10, 20, 30, 40, 50];
    let mut rng = MockRng { value: 10 };

    #[should_panic(expected = "Uniform::new(0, {}) somehow returned {}", slice.len())]
    let _result = sample(slice, &mut rng);
}

#[test]
fn test_sample_empty_slice() {
    struct MockRng;

    impl crate::Rng for MockRng {
        fn gen_range(&mut self, start: usize, end: usize) -> usize {
            // This should never be called since the slice is empty.
            panic!("This should never happen");
        }
    }

    let slice: &[i32] = &[];
    let mut rng = MockRng;

    // Safety: The function should not be called on an empty slice
    #[should_panic(expected = "slice is empty")]
    let _result = sample(slice, &mut rng);
}

