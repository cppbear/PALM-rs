// Answer 0

#[test]
fn test_append_value_with_existing_links() {
    struct Bucket<T> {
        links: Option<Links>,
    }

    struct ExtraValue<T> {
        value: T,
        prev: Link,
        next: Link,
    }

    struct Links {
        tail: usize,
        next: usize,
    }

    enum Link {
        Extra(usize),
        Entry(usize),
    }

    let entry_idx = 0;
    
    let mut extra: Vec<ExtraValue<i32>> = Vec::new();
    let mut entry = Bucket {
        links: Some(Links { tail: 0, next: 0 }),
    };
    
    append_value(entry_idx, &mut entry, &mut extra, 42);
    
    assert_eq!(extra.len(), 1);
    assert_eq!(extra[0].value, 42);
    match &entry.links {
        Some(links) => {
            assert_eq!(links.tail, 0);
            assert_eq!(links.next, 0);
        }
        None => panic!("Links should not be None"),
    }
}

#[test]
fn test_append_value_with_no_existing_links() {
    struct Bucket<T> {
        links: Option<Links>,
    }

    struct ExtraValue<T> {
        value: T,
        prev: Link,
        next: Link,
    }

    struct Links {
        tail: usize,
        next: usize,
    }

    enum Link {
        Extra(usize),
        Entry(usize),
    }

    let entry_idx = 1;

    let mut extra: Vec<ExtraValue<i32>> = Vec::new();
    let mut entry = Bucket {
        links: None,
    };

    append_value(entry_idx, &mut entry, &mut extra, 99);

    assert_eq!(extra.len(), 1);
    assert_eq!(extra[0].value, 99);
    match &entry.links {
        Some(links) => {
            assert_eq!(links.tail, 0);
            assert_eq!(links.next, 0);
        }
        None => panic!("Links should not be None"),
    }
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_append_value_with_invalid_link_access() {
    struct Bucket<T> {
        links: Option<Links>,
    }

    struct ExtraValue<T> {
        value: T,
        prev: Link,
        next: Link,
    }

    struct Links {
        tail: usize,
        next: usize,
    }

    enum Link {
        Extra(usize),
        Entry(usize),
    }

    let entry_idx = 0;

    let mut extra: Vec<ExtraValue<i32>> = Vec::new();
    let mut entry = Bucket {
        links: Some(Links { tail: 5, next: 0 }), // Set invalid tail for panic
    };

    append_value(entry_idx, &mut entry, &mut extra, 10);
}

