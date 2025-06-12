// Answer 0

#[test]
fn test_append_value_empty_links() {
    struct Bucket<T> {
        links: Option<Links>,
    }

    struct ExtraValue<T> {
        value: T,
        prev: Link,
        next: Link,
    }

    struct Links {
        next: usize,
        tail: usize,
    }

    enum Link {
        Extra(usize),
        Entry(usize),
    }

    let mut entry: Bucket<i32> = Bucket { links: None };
    let mut extra: Vec<ExtraValue<i32>> = Vec::new();
    let value = 42;
    let entry_idx = 0;

    append_value(entry_idx, &mut entry, &mut extra, value);

    assert_eq!(extra.len(), 1);
    assert_eq!(extra[0].value, value);
    match entry.links {
        Some(ref links) => {
            assert_eq!(links.next, 0);
            assert_eq!(links.tail, 0);
        }
        None => panic!("Expected links to be Some after appending value"),
    }
}

