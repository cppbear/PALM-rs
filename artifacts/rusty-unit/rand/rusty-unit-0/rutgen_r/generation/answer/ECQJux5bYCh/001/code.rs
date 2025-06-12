// Answer 0

#[test]
fn test_fill_with_valid_rng() {
    struct TestFill {
        data: Vec<u32>,
    }

    impl TestFill {
        fn new(size: usize) -> Self {
            TestFill {
                data: vec![0; size],
            }
        }

        fn iter_mut(&mut self) -> std::slice::IterMut<u32> {
            self.data.iter_mut()
        }
    }

    struct MockRng {
        value: u32,
    }

    impl MockRng {
        fn new(value: u32) -> Self {
            MockRng { value }
        }

        fn random(&mut self) -> u32 {
            self.value
        }
    }

    let mut fill = TestFill::new(5);
    let mut rng = MockRng::new(42);

    fill.fill(&mut rng);

    assert_eq!(fill.data, vec![42, 42, 42, 42, 42]);
}

#[test]
#[should_panic]
fn test_fill_with_empty_iter_mut() {
    struct EmptyTestFill;

    impl EmptyTestFill {
        fn iter_mut(&mut self) -> std::slice::IterMut<u32> {
            std::slice::from_ref(&0).iter_mut()
        }
    }

    let mut fill = EmptyTestFill;
    let mut rng = MockRng::new(42);

    fill.fill(&mut rng); // This will panic with the current implementation since iter_mut is not yielding elements.
}

