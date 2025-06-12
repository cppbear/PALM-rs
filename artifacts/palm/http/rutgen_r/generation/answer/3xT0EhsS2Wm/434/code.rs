// Answer 0

fn test_remove_extra_value() {
    #[derive(Debug, Clone)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct RawLink {
        next: usize,
        tail: usize,
    }

    #[derive(Debug)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }

    let mut raw_links: Vec<Option<RawLink>> = vec![
        Some(RawLink { next: 1, tail: 1 }),
        Some(RawLink { next: 0, tail: 0 }),
    ];

    let mut extra_values: Vec<ExtraValue<i32>> = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(0), value: 10 },
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(1), value: 20 }
    ];

    let idx = 0;

    // This case ensures that all constraints are validated and reachable.
    let result = remove_extra_value(raw_links.clone(), &mut extra_values.clone(), idx);
    
    assert_eq!(result.value, 10);
    assert_eq!(extra_values.len(), 1); // Validate that it removed one entry
}

fn test_remove_extra_value_boundary() {
    #[derive(Debug, Clone)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct RawLink {
        next: usize,
        tail: usize,
    }

    #[derive(Debug)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }

    let mut raw_links: Vec<Option<RawLink>> = vec![
        Some(RawLink { next: 1, tail: 1 }),
    ];

    let mut extra_values: Vec<ExtraValue<i32>> = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(0), value: 30 },
    ];

    let idx = 0;

    // Test attempt to remove the last remaining ExtraValue
    let result = remove_extra_value(raw_links.clone(), &mut extra_values.clone(), idx);
    
    assert_eq!(result.value, 30);
    assert_eq!(extra_values.len(), 0); // Validate it removed the last entry
}

fn test_remove_extra_value_panic_conditions() {
    #[derive(Debug, Clone)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct RawLink {
        next: usize,
        tail: usize,
    }

    #[derive(Debug)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }

    let mut raw_links: Vec<Option<RawLink>> = vec![None]; // No raw link to ensure panic

    let mut extra_values: Vec<ExtraValue<i32>> = vec![
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(0), value: 10 }, // This should panic due to invalid entry
    ];

    let idx = 0;

    let result = std::panic::catch_unwind(|| {
        remove_extra_value(raw_links.clone(), &mut extra_values.clone(), idx)
    });

    assert!(result.is_err()); // Confirm that a panic was triggered
}

fn test_remove_extra_value_panic_on_invalid_index() {
    #[derive(Debug, Clone)]
    enum Link {
        Entry(usize),
        Extra(usize),
    }

    #[derive(Debug)]
    struct RawLink {
        next: usize,
        tail: usize,
    }

    #[derive(Debug)]
    struct ExtraValue<T> {
        prev: Link,
        next: Link,
        value: T,
    }

    let mut raw_links: Vec<Option<RawLink>> = vec![
        Some(RawLink { next: 1, tail: 1 }),
    ];

    let mut extra_values: Vec<ExtraValue<i32>> = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Entry(0), value: 10 },
    ];

    let idx = 1; // Invalid index for removal

    let result = std::panic::catch_unwind(|| {
        remove_extra_value(raw_links.clone(), &mut extra_values.clone(), idx)
    });

    assert!(result.is_err()); // Confirm that a panic was triggered due to an invalid index
}

