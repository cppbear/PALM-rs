// Answer 0

#[derive(Default)]
struct TestStruct {
    data: Vec<u32>,
}

impl TestStruct {
    fn iter_mut(&mut self) -> std::slice::IterMut<u32> {
        self.data.iter_mut()
    }
}

trait Rng {
    fn random(&mut self) -> u32;
}

struct MockRng {
    counter: u32,
}

impl MockRng {
    fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Rng for MockRng {
    fn random(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}

#[test]
fn test_fill_with_empty_data() {
    let mut rng = MockRng::new();
    let mut test_struct = TestStruct::default(); // empty vectors

    test_struct.fill(&mut rng); // should not panic
    assert_eq!(test_struct.data.len(), 0); // still empty
}

#[test]
fn test_fill_with_non_empty_data() {
    let mut rng = MockRng::new();
    let mut test_struct = TestStruct { 
        data: vec![0; 5], // pre-fill with zeros
    };

    test_struct.fill(&mut rng); // should not panic
    assert_eq!(test_struct.data, vec![1, 2, 3, 4, 5]); // checking values filled
}

#[test]
#[should_panic]
fn test_fill_with_no_mutable_borrow() {
    let rng = &mut MockRng::new(); 
    let test_struct = TestStruct { 
        data: vec![], // empty data will trigger a panic if mutability is checked
    };

    test_struct.iter_mut(); // invokes mutable borrow checking
    test_struct.fill(rng); // should panic due to the non-mutable borrow
}

