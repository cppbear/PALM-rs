// Answer 0

#[derive(Debug)]
struct RawLinks<T>(Vec<Option<T>>);

#[derive(Debug, Clone, Copy)]
enum Link {
    Entry(usize),
    Extra(usize),
}

#[derive(Debug)]
struct ExtraValue<T> {
    prev: Link,
    next: Link,
    _marker: std::marker::PhantomData<T>,
}

#[test]
fn test_remove_extra_value_entry_entry() {
    let mut raw_links = RawLinks(vec![Some(()), Some(()), Some(())]);
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Entry(2), _marker: std::marker::PhantomData },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(2), _marker: std::marker::PhantomData },
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(0), _marker: std::marker::PhantomData },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 1);
    assert_eq!(result.prev, Link::Extra(0));
    assert_eq!(result.next, Link::Extra(2));
}

#[test]
fn test_remove_extra_value_entry_extra() {
    let mut raw_links = RawLinks(vec![Some(()), Some(())]);
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(1), next: Link::Extra(1), _marker: std::marker::PhantomData },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(0), _marker: std::marker::PhantomData },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 0);
    assert_eq!(result.prev, Link::Entry(1));
    assert_eq!(result.next, Link::Extra(1));
}

#[test]
#[should_panic]
fn test_remove_extra_value_out_of_bounds() {
    let mut raw_links = RawLinks(vec![Some(()), Some(())]);
    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(0), _marker: std::marker::PhantomData },
    ];

    let _ = remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
fn test_remove_extra_value_extra_extra() {
    let mut raw_links = RawLinks(vec![Some(()), Some(()), Some(())]);
    let mut extra_values = vec![
        ExtraValue { prev: Link::Extra(1), next: Link::Extra(2), _marker: std::marker::PhantomData },
        ExtraValue { prev: Link::Extra(0), next: Link::Entry(1), _marker: std::marker::PhantomData },
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1), _marker: std::marker::PhantomData },
    ];

    let result = remove_extra_value(raw_links, &mut extra_values, 0);
    assert_eq!(result.prev, Link::Extra(1));
    assert_eq!(result.next, Link::Extra(2));
}

