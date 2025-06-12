// Answer 0

#[test]
fn test_remove_extra_value() {
    struct RawLinks<T> {
        links: Vec<Option<Link<T>>>,
    }
    
    impl<T> RawLinks<T> {
        fn new(size: usize) -> Self {
            RawLinks {
                links: vec![None; size],
            }
        }
        
        fn get(&self, idx: usize) -> &Option<Link<T>> {
            &self.links[idx]
        }

        fn set(&mut self, idx: usize, value: Option<Link<T>>) {
            self.links[idx] = value;
        }
    }

    enum Link<T> {
        Entry(usize),
        Extra(usize),
    }

    struct ExtraValue<T> {
        prev: Link<T>,
        next: Link<T>,
    }

    let mut raw_links = RawLinks::new(3);
    raw_links.set(0, Some(Link::Entry(1)));
    raw_links.set(1, Some(Link::Extra(2)));
    raw_links.set(2, Some(Link::Entry(0)));

    let mut extra_values = vec![
        ExtraValue { prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { prev: Link::Extra(0), next: Link::Extra(2) },
        ExtraValue { prev: Link::Extra(1), next: Link::Entry(0) },
    ];

    let idx_to_remove = 1;
    let removed_extra = remove_extra_value(raw_links, &mut extra_values, idx_to_remove);

    assert_eq!(removed_extra.prev, Link::Extra(0));
    assert_eq!(removed_extra.next, Link::Extra(2));
    assert!(matches!(extra_values[0].next, Link::Extra(2)));
    assert!(matches!(extra_values[2].prev, Link::Extra(0)));
}

