// Answer 0

#[test]
fn test_fill_with_rng() {
    use rand::Rng;

    struct TestStruct {
        data: Vec<u32>,
    }

    impl TestStruct {
        fn iter_mut(&mut self) -> std::slice::IterMut<u32> {
            self.data.iter_mut()
        }
    }

    struct MockRng {
        values: Vec<u32>,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u32 {
            self.values.remove(0)
        }
    }

    let mut rng = MockRng { values: vec![1, 2, 3, 4, 5] };
    let mut test_struct = TestStruct { data: vec![0; 5] };

    test_struct.fill(&mut rng);

    assert_eq!(test_struct.data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_fill_empty_struct() {
    use rand::Rng;

    struct TestStruct {
        data: Vec<u32>,
    }

    impl TestStruct {
        fn iter_mut(&mut self) -> std::slice::IterMut<u32> {
            self.data.iter_mut()
        }
    }

    struct MockRng {
        values: Vec<u32>,
    }

    impl Rng for MockRng {
        fn random(&mut self) -> u32 {
            self.values.remove(0)
        }
    }

    let mut rng = MockRng { values: vec![] };
    let mut test_struct = TestStruct { data: vec![] };

    test_struct.fill(&mut rng);

    assert_eq!(test_struct.data, vec![]);
}

