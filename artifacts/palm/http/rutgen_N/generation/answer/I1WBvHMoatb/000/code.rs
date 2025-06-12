// Answer 0

#[test]
fn test_drain_all_extra_values_with_no_extra() {
    struct RawLinks<T> {
        // fields as per requirement
    }

    struct ExtraValue<T> {
        value: T,
        next: Link,
    }

    enum Link {
        Extra(usize),
        End,
    }

    fn remove_extra_value<T>(
        _raw_links: RawLinks<T>,
        extra_values: &mut Vec<ExtraValue<T>>,
        _head: usize,
    ) -> ExtraValue<T> {
        extra_values.remove(0) // Dummy implementation for the purpose of the test
    }

    let raw_links = RawLinks {};
    let mut extra_values = vec![ExtraValue { value: 1, next: Link::End }];
    let head = 0;

    let result = drain_all_extra_values(raw_links, &mut extra_values, head);
    assert_eq!(result, vec![1]);
}

#[test]
fn test_drain_all_extra_values_with_multiple_extra() {
    struct RawLinks<T> {
        // fields as per requirement
    }

    struct ExtraValue<T> {
        value: T,
        next: Link,
    }

    enum Link {
        Extra(usize),
        End,
    }

    fn remove_extra_value<T>(
        _raw_links: RawLinks<T>,
        extra_values: &mut Vec<ExtraValue<T>>,
        head: usize,
    ) -> ExtraValue<T> {
        extra_values.remove(head) // Dummy implementation for the purpose of the test
    }

    let raw_links = RawLinks {};
    let mut extra_values = vec![
        ExtraValue { value: 1, next: Link::Extra(1) },
        ExtraValue { value: 2, next: Link::Extra(2) },
        ExtraValue { value: 3, next: Link::End },
    ];
    let head = 0;

    let result = drain_all_extra_values(raw_links, &mut extra_values, head);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_drain_all_extra_values_with_empty_extra_values() {
    struct RawLinks<T> {
        // fields as per requirement
    }

    struct ExtraValue<T> {
        value: T,
        next: Link,
    }

    enum Link {
        Extra(usize),
        End,
    }

    fn remove_extra_value<T>(
        _raw_links: RawLinks<T>,
        _extra_values: &mut Vec<ExtraValue<T>>,
        _head: usize,
    ) -> ExtraValue<T> {
        panic!("This should not be called since extra_values is empty.");
    }

    let raw_links = RawLinks {};
    let mut extra_values = Vec::new();
    let head = 0;

    let result: Vec<i32> = drain_all_extra_values(raw_links, &mut extra_values, head);
    assert_eq!(result, Vec::<i32>::new());
}

