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

    fn remove_extra_value<T>(
        raw_links: &RawLinks<T>,
        extra_values: &mut Vec<ExtraValue<T>>,
        head: usize,
    ) -> ExtraValue<T> {
        extra_values.remove(head)
    }

    let raw_links = RawLinks {
        links: vec![Link::Extra(1), Link::Extra(2), Link::End],
    };

    let mut extra_values = vec![
        ExtraValue {
            value: "first",
            next: Link::Extra(1),
        },
        ExtraValue {
            value: "second",
            next: Link::Extra(2),
        },
        ExtraValue {
            value: "third",
            next: Link::End,
        },
    ];

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);
    assert_eq!(result, vec!["first", "second", "third"]);
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

    fn remove_extra_value<T>(
        raw_links: &RawLinks<T>,
        extra_values: &mut Vec<ExtraValue<T>>,
        head: usize,
    ) -> ExtraValue<T> {
        extra_values.remove(head)
    }

    let raw_links = RawLinks {
        links: vec![Link::Extra(0), Link::End],
    };

    let mut extra_values = vec![
        ExtraValue {
            value: "only_extra",
            next: Link::End,
        },
    ];

    let result = drain_all_extra_values(raw_links, &mut extra_values, 0);
    assert_eq!(result, vec!["only_extra"]);
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

    fn remove_extra_value<T>(
        raw_links: &RawLinks<T>,
        extra_values: &mut Vec<ExtraValue<T>>,
        head: usize,
    ) -> ExtraValue<T> {
        extra_values.remove(head)
    }

    let raw_links = RawLinks {
        links: vec![Link::Extra(0), Link::End],
    };

    let mut extra_values: Vec<ExtraValue<&str>> = Vec::new();

    drain_all_extra_values(raw_links, &mut extra_values, 0);
}

