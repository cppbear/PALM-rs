// Answer 0

#[derive(Default)]
struct TestSet<T> {
    elements: Vec<T>,
}

impl<T: PartialEq> TestSet<T> {
    pub fn replace(&mut self, value: T) -> Option<T> {
        if let Some(pos) = self.elements.iter().position(|x| *x == value) {
            Some(std::mem::replace(&mut self.elements[pos], value))
        } else {
            self.elements.push(value);
            None
        }
    }
}

#[test]
fn test_replace_with_existing_value() {
    let mut set: TestSet<i32> = TestSet::default();
    set.replace(10);
    let replaced = set.replace(10);
    assert_eq!(replaced, Some(10));
    assert_eq!(set.elements, vec![10]);
}

#[test]
fn test_replace_with_new_value() {
    let mut set: TestSet<i32> = TestSet::default();
    let replaced = set.replace(20);
    assert_eq!(replaced, None);
    assert_eq!(set.elements, vec![20]);
}

#[test]
fn test_replace_value_insertion_order() {
    let mut set: TestSet<i32> = TestSet::default();
    set.replace(30);
    set.replace(40);
    set.replace(30);
    assert_eq!(set.elements, vec![40, 30]);
}

#[test]
fn test_replace_multiple_replacements() {
    let mut set: TestSet<i32> = TestSet::default();
    set.replace(50);
    set.replace(60);
    let replaced = set.replace(50);
    assert_eq!(replaced, Some(50));
    let replaced_new = set.replace(70);
    assert_eq!(replaced_new, None);
    assert_eq!(set.elements, vec![60, 70]);
}

