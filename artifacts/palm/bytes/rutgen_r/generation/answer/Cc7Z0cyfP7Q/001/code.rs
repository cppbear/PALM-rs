// Answer 0

fn remaining_mut(a: usize, b: usize) -> usize {
    a.saturating_add(b)
}

struct A {
    a: usize,
}

impl A {
    fn remaining_mut(&self) -> usize {
        self.a
    }
}

struct B {
    b: usize,
}

impl B {
    fn remaining_mut(&self) -> usize {
        self.b
    }
}

struct Chain<'a> {
    a: &'a A,
    b: &'a B,
}

impl<'a> Chain<'a> {
    fn remaining_mut(&self) -> usize {
        self.a.remaining_mut().saturating_add(self.b.remaining_mut())
    }
}

#[test]
fn test_remaining_mut() {
    let a = A { a: 5 };
    let b = B { b: 10 };
    let chain = Chain { a: &a, b: &b };
    assert_eq!(chain.remaining_mut(), 15);
}

#[test]
fn test_remaining_mut_zero() {
    let a = A { a: 0 };
    let b = B { b: 0 };
    let chain = Chain { a: &a, b: &b };
    assert_eq!(chain.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_large_values() {
    let a = A { a: usize::MAX };
    let b = B { b: usize::MAX };
    let chain = Chain { a: &a, b: &b };
    assert_eq!(chain.remaining_mut(), usize::MAX); // should not panic due to saturating_add
}

#[test]
fn test_remaining_mut_large_with_zero() {
    let a = A { a: usize::MAX };
    let b = B { b: 0 };
    let chain = Chain { a: &a, b: &b };
    assert_eq!(chain.remaining_mut(), usize::MAX);
}

