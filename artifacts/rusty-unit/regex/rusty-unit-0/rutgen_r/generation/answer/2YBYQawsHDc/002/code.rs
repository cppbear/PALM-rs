// Answer 0

#[test]
fn test_pop_with_non_empty_alternation() {
    // Define a minimal structure Frame to represent the possible variants
    enum Frame<'a> {
        Repetition(Vec<&'a str>),
        Group(Vec<&'a str>),
        Concat { head: &'a str, tail: &'a [&'a str] },
        Alternation { head: &'a str, tail: &'a [&'a str] },
    }

    // Prepare test input with non-empty tail
    let tail: Vec<&str> = vec!["first", "second", "third"];
    let induct = Frame::Alternation { head: "dummy_head", tail: &tail };

    // Call the method under test
    let result = pop(induct);

    // Assert the expected result
    if let Some(Frame::Alternation { head, tail }) = result {
        assert_eq!(head, &tail[0]);
        assert_eq!(tail.len(), 2); // Should return two elements in the tail
        assert_eq!(tail[0], &tail[1..][0]); // Check that we have the expected elements
    } else {
        panic!("Expected Some(Frame::Alternation) but got None");
    }
}

// Required to define the function itself since it's being tested here
fn pop<'a>(induct: Frame<'a>) -> Option<Frame<'a>> {
    match induct {
        Frame::Repetition(_) => None,
        Frame::Group(_) => None,
        Frame::Concat { tail, .. } => {
            if tail.is_empty() {
                None
            } else {
                Some(Frame::Concat {
                    head: &tail[0],
                    tail: &tail[1..],
                })
            }
        }
        Frame::Alternation { tail, .. } => {
            if tail.is_empty() {
                None
            } else {
                Some(Frame::Alternation {
                    head: &tail[0],
                    tail: &tail[1..],
                })
            }
        }
    }
}

