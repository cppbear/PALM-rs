// Answer 0

#[derive(Debug)]
struct Borrowed;

struct Example {
    borrowed: Borrowed,
}

impl Example {
    fn borrow(&self) -> &Borrowed {
        &self.borrowed
    }
}

#[test]
fn test_borrow() {
    let example = Example {
        borrowed: Borrowed,
    };
    let borrowed_ref = example.borrow();
    assert_eq!(format!("{:?}", borrowed_ref), "Borrowed");
}

