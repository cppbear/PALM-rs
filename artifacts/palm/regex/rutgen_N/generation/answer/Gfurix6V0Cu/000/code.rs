// Answer 0

#[derive(Default)]
struct ExampleStruct {
    v: Vec<u8>,
}

impl ExampleStruct {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.v
    }
}

#[test]
fn test_deref_mut_initialization() {
    let mut example = ExampleStruct::default();
    let vec_ref = example.deref_mut();
    assert!(vec_ref.is_empty());
}

#[test]
fn test_deref_mut_add_element() {
    let mut example = ExampleStruct::default();
    {
        let vec_ref = example.deref_mut();
        vec_ref.push(42);
    }
    assert_eq!(example.v.len(), 1);
    assert_eq!(example.v[0], 42);
}

#[test]
fn test_deref_mut_clear_elements() {
    let mut example = ExampleStruct::default();
    {
        let vec_ref = example.deref_mut();
        vec_ref.push(1);
        vec_ref.push(2);
        vec_ref.push(3);
        vec_ref.clear();
    }
    assert!(example.v.is_empty());
}

