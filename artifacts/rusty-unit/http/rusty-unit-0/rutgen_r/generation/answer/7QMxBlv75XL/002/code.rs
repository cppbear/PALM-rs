// Answer 0

fn remove_extra_value(head: usize) -> Link {
    // Mock implementation for test purposes
    if head < 5 {
        Link::Extra(head + 1)
    } else {
        Link::None
    }
}

enum Link {
    Extra(usize),
    None,
}

struct TestStruct {
    head: usize,
}

impl TestStruct {
    fn remove_extra_value(&mut self, head: usize) -> Link {
        remove_extra_value(head)
    }

    fn remove_all_extra_values(&mut self, mut head: usize) {
        loop {
            let extra = self.remove_extra_value(head);
            if let Link::Extra(idx) = extra {
                head = idx;
            } else {
                break;
            }
        }
    }
}

#[test]
fn test_remove_all_extra_values() {
    let mut test_struct = TestStruct { head: 0 };
    test_struct.remove_all_extra_values(0);
    // Assert expected outcome, in this scenario, we expect the head to become 5
    assert_eq!(test_struct.head, 5);
}

#[test]
fn test_remove_all_extra_values_with_boundary() {
    let mut test_struct = TestStruct { head: 4 };
    test_struct.remove_all_extra_values(4);
    // Assert expected outcome, head should now be 5 after traversing
    assert_eq!(test_struct.head, 5);
}

#[test]
fn test_remove_all_extra_values_no_extra() {
    let mut test_struct = TestStruct { head: 5 };
    test_struct.remove_all_extra_values(5);
    // Assert expected outcome, head should remain at 5 as no extra values exist
    assert_eq!(test_struct.head, 5);
}

