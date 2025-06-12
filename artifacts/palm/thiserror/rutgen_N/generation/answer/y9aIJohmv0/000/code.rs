// Answer 0

#[derive(Clone, Copy)]
struct DisplayExample;

impl DisplayExample {
    fn as_display(&self) -> Self {
        *self
    }
}

#[test]
fn test_as_display() {
    let example = DisplayExample;
    let result = example.as_display();
    assert_eq!(result, example);
}

