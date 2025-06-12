// Answer 0

#[test]
fn test_choice_empty_iterator() {
    let empty_vec: Vec<i32> = Vec::new();
    let result = choice(empty_vec);
    assert_eq!(result, None);
}

#[test]
fn test_choice_single_element() {
    let single_elem_vec = vec![42];
    let result = choice(single_elem_vec);
    assert_eq!(result, Some(42));
}

#[test]
fn test_choice_multiple_elements() {
    let multiple_elem_vec = vec![1, 2, 3, 4, 5];
    let result = choice(multiple_elem_vec);
    assert!(result.is_some());
}

#[should_panic]
fn test_choice_invalid_length() {
    struct InvalidLengthIter {
        items: Vec<i32>,
    }

    impl IntoIterator for InvalidLengthIter {
        type Item = i32;
        type IntoIter = std::vec::IntoIter<i32>;

        fn into_iter(self) -> Self::IntoIter {
            self.items.into_iter()
        }
    }

    let iter = InvalidLengthIter { items: vec![1, 2, 3] };
    let result = choice(iter);
    assert!(result.is_some());
}

