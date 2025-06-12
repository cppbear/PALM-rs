// Answer 0

#[test]
fn test_drain_all_extra_values_with_multiple_extra_links() {
    struct RawLinks<T> {
        links: Vec<Link<T>>,
    }

    enum Link<T> {
        Extra(usize),
        End,
    }

    struct ExtraValue<T> {
        value: T,
        next: Link<T>,
    }

    fn remove_extra_value<T>(raw_links: &RawLinks<T>, extra_values: &mut Vec<ExtraValue<T>>, head: usize) -> ExtraValue<T> {
        extra_values.remove(head)
    }

    let mut extra_values = vec![
        ExtraValue { value: "value1", next: Link::Extra(1) },
        ExtraValue { value: "value2", next: Link::Extra(2) },
        ExtraValue { value: "value3", next: Link::End },
    ];

    let raw_links = RawLinks { links: vec![Link::Extra(0), Link::Extra(1), Link::Extra(2)] };

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);

    assert_eq!(result, vec!["value1", "value2", "value3"]);
}

#[test]
fn test_drain_all_extra_values_with_single_extra_link() {
    struct RawLinks<T> {
        links: Vec<Link<T>>,
    }

    enum Link<T> {
        Extra(usize),
        End,
    }

    struct ExtraValue<T> {
        value: T,
        next: Link<T>,
    }

    fn remove_extra_value<T>(raw_links: &RawLinks<T>, extra_values: &mut Vec<ExtraValue<T>>, head: usize) -> ExtraValue<T> {
        extra_values.remove(head)
    }

    let mut extra_values = vec![
        ExtraValue { value: "single_value", next: Link::End },
    ];

    let raw_links = RawLinks { links: vec![Link::Extra(0)] };

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);

    assert_eq!(result, vec!["single_value"]);
}

#[test]
#[should_panic]
fn test_drain_all_extra_values_with_empty_extra_values() {
    struct RawLinks<T> {
        links: Vec<Link<T>>,
    }

    enum Link<T> {
        Extra(usize),
        End,
    }

    struct ExtraValue<T> {
        value: T,
        next: Link<T>,
    }

    fn remove_extra_value<T>(raw_links: &RawLinks<T>, extra_values: &mut Vec<ExtraValue<T>>, head: usize) -> ExtraValue<T> {
        extra_values.remove(head)
    }

    let raw_links = RawLinks { links: vec![Link::Extra(0)] };
    let mut extra_values: Vec<ExtraValue<&str>> = Vec::new();

    drain_all_extra_values(raw_links, &mut extra_values, 0);
}

