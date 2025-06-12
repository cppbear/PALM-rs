// Answer 0

#[derive(Default)]
struct Literals {
    limit_size: usize,
    members: Vec<String>,
}

impl Literals {
    pub fn set_limit_size(&mut self, size: usize) -> &mut Literals {
        self.limit_size = size;
        self
    }
}

#[test]
fn test_set_limit_size_increases_limit() {
    let mut literals = Literals::default();
    literals.set_limit_size(100);
    assert_eq!(literals.limit_size, 100);
}

#[test]
fn test_set_limit_size_reduces_limit() {
    let mut literals = Literals::default();
    literals.set_limit_size(50);
    assert_eq!(literals.limit_size, 50);
}

#[test]
fn test_set_limit_size_leaves_existing_members_untouched() {
    let mut literals = Literals::default();
    literals.members.push("a".to_string());
    literals.set_limit_size(200);
    assert_eq!(literals.members.len(), 1);
    assert_eq!(literals.limit_size, 200);
}

