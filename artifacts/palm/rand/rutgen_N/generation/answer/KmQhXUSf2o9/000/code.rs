// Answer 0

#[derive(Debug)]
struct R(u32);

struct UnwrapMut<'b, R>(&'b mut R);

impl R {
    pub fn new(value: u32) -> Self {
        R(value)
    }
    
    pub fn re<'b>(&'b mut self) -> UnwrapMut<'b, R> {
        UnwrapMut(self)
    }
}

#[test]
fn test_reborrow() {
    let mut r = R::new(42);
    {
        let reborrowed = r.re();
        assert_eq!(reborrowed.0.0, 42);
    }

    let reborrowed_again = r.re();
    assert_eq!(reborrowed_again.0.0, 42);
}

#[test]
fn test_reborrow_mut() {
    let mut r = R::new(100);
    {
        let reborrowed_mut = r.re();
        reborrowed_mut.0.0 += 1;
        assert_eq!(reborrowed_mut.0.0, 101);
    }

    let reborrowed_mut_again = r.re();
    assert_eq!(reborrowed_mut_again.0.0, 101);
}

