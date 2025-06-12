// Answer 0

#[derive(Debug)]
struct ExampleStruct {
    insts: Vec<i32>,
}

impl ExampleStruct {
    fn deref(&self) -> &Vec<i32> {
        &*self.insts
    }
}

#[test]
fn test_deref_with_non_empty_vector() {
    let example = ExampleStruct { insts: vec![1, 2, 3] };
    let result = example.deref();
    assert_eq!(result, &vec![1, 2, 3]);
}

#[test]
fn test_deref_with_empty_vector() {
    let example = ExampleStruct { insts: vec![] };
    let result = example.deref();
    assert_eq!(result, &vec![]);
}

#[test]
fn test_deref_with_large_vector() {
    let example = ExampleStruct { insts: (0..1000).collect() };
    let result = example.deref();
    assert_eq!(result, &(0..1000).collect::<Vec<i32>>());
}

