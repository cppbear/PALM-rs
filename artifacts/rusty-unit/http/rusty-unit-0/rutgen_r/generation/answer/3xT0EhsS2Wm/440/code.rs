// Answer 0

#[test]
#[should_panic]
fn test_remove_extra_value_panic_index_equal_length() {
    struct RawLinks<T> {
        links: Vec<Option<LinkData<T>>>,
    }

    struct LinkData<T> {
        next: Link,
        tail: usize,
    }

    enum Link {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }

    let raw_links = RawLinks {
        links: vec![Some(LinkData { next: Link::Entry(0), tail: 1 })],
    };

    let extra_values: Vec<ExtraValue<u32>> = vec![ExtraValue { prev: Link::Entry(0), next: Link::Entry(0), value: 1 }];

    let idx = extra_values.len(); // This will be equal to the length of extra_values, causing the panic.
    
    let _ = remove_extra_value(raw_links, &mut extra_values.clone(), idx);
}

