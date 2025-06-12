// Answer 0

#[derive(Default)]
struct Container<A> {
    alloc: A,
}

impl<A> Container<A> {
    pub fn new(alloc: A) -> Self {
        Container { alloc }
    }

    pub fn allocator(&self) -> &A {
        &self.alloc
    }
}

#[test]
fn test_allocator() {
    let allocator = Vec::<u8>::new();
    let container = Container::new(allocator);
    assert_eq!(container.allocator().len(), 0);
}

#[test]
fn test_allocator_non_empty() {
    let allocator = Vec::<u8>::from(vec![1, 2, 3]);
    let container = Container::new(allocator);
    assert_eq!(container.allocator().len(), 3);
}

#[test]
fn test_allocator_boundary_empty() {
    let allocator: Vec<u8> = Vec::new();
    let container = Container::new(allocator);
    assert!(container.allocator().is_empty());
}

#[test]
fn test_allocator_boundary_small() {
    let allocator = Vec::<u8>::from(vec![42]);
    let container = Container::new(allocator);
    assert_eq!(container.allocator().len(), 1);
}

