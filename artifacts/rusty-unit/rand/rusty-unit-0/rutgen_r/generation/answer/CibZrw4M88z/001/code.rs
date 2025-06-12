// Answer 0

#[derive(Debug)]
struct Borrowed;

struct Uniform {
    borrowed: Borrowed,
}

impl Uniform {
    fn borrow(&self) -> &Borrowed {
        self.borrowed
    }
}

#[test]
fn test_borrow() {
    let uniform = Uniform {
        borrowed: Borrowed,
    };
    let borrowed_ref = uniform.borrow();
    
    assert!(std::ptr::eq(borrowed_ref, &uniform.borrowed));
}

#[test]
fn test_borrow_multiple_calls() {
    let uniform = Uniform {
        borrowed: Borrowed,
    };
    let borrowed_ref1 = uniform.borrow();
    let borrowed_ref2 = uniform.borrow();

    assert!(std::ptr::eq(borrowed_ref1, borrowed_ref2));
}

