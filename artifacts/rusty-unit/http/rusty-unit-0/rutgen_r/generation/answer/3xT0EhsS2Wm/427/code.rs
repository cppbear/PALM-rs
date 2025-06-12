// Answer 0

#[test]
fn test_remove_extra_value_valid_case() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }
    struct Links<T> {
        next: usize,
        tail: usize,
        _marker: std::marker::PhantomData<T>,
    }
    
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![Some(Links { next: 1, tail: 1, _marker: std::marker::PhantomData }),
                                       Some(Links { next: 0, tail: 0, _marker: std::marker::PhantomData })]);

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1), value: "A" },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(0), value: "B" },
        ExtraValue { prev: Link::Entry(1), next: Link::Extra(1), value: "C" },
    ];

    let idx = 1;

    let result = remove_extra_value(raw_links, &mut extra_values, idx);
    assert_eq!(result.value, "B");
    assert_eq!(extra_values.len(), 2);
}

#[test]
#[should_panic]
fn test_remove_extra_value_idx_out_of_bounds() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }
    struct Links<T> {
        next: usize,
        tail: usize,
        _marker: std::marker::PhantomData<T>,
    }

    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![Some(Links { next: 1, tail: 1, _marker: std::marker::PhantomData })]);
    let mut extra_values = vec![ExtraValue { prev: Link::Entry(0), next: Link::Entry(0), value: "A" }];

    let idx = 1; // Out of bounds index
    remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
#[should_panic]
fn test_remove_extra_value_inconsistent_links() {
    struct RawLinks<T>(Vec<Option<Links<T>>>);
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }
    struct Links<T> {
        next: usize,
        tail: usize,
        _marker: std::marker::PhantomData<T>,
    }

    enum Link {
        Entry(usize),
        Extra(usize),
    }

    let mut raw_links = RawLinks(vec![None]);
    let mut extra_values = vec![ExtraValue { prev: Link::Entry(0), next: Link::Entry(0), value: "A" }];

    let idx = 0;
    remove_extra_value(raw_links, &mut extra_values, idx);
}

