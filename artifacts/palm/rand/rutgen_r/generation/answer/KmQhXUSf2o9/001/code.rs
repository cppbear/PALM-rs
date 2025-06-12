// Answer 0

#[derive(Debug)]
struct R {
    value: i32,
}

struct UnwrapMut<'a>(&'a mut R);

impl R {
    pub fn new(value: i32) -> Self {
        R { value }
    }

    pub fn re<'b>(&'b mut self) -> UnwrapMut<'b> {
        UnwrapMut(self)
    }
}

#[test]
fn test_reborrow_valid() {
    let mut r = R::new(10);
    let unwrap_mut = r.re();
    assert_eq!(unwrap_mut.0.value, 10);
}

#[test]
fn test_reborrow_multiple() {
    let mut r1 = R::new(20);
    let mut r2 = R::new(30);
    
    {
        let unwrap_mut1 = r1.re();
        let unwrap_mut2 = r2.re();
        assert_eq!(unwrap_mut1.0.value, 20);
        assert_eq!(unwrap_mut2.0.value, 30);
    }

    // Check that r1 and r2 can still be accessed after reborrowing
    assert_eq!(r1.value, 20);
    assert_eq!(r2.value, 30);
}

#[should_panic(expected = "attempt to access a freed value")]
#[test]
fn test_reborrow_invalid() {
    let mut r = R::new(40);
    let _unwrap_mut = r.re();
    let _unwrap_mut2 = r; // This will trigger panic as r is already reborrowed
}

