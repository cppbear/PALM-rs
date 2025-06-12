// Answer 0

#[derive(Debug)]
struct Link {
    next: LinkType,
}

#[derive(Debug)]
enum LinkType {
    Extra(usize),
    None,
}

struct List {
    links: Vec<Link>,
}

impl List {
    fn remove_extra_value(&mut self, _head: usize) -> Link {
        // Dummy implementation for testing
        if _head < self.links.len() {
            self.links.remove(_head)
        } else {
            Link { next: LinkType::None }
        }
    }

    fn new(links: Vec<Link>) -> Self {
        List { links }
    }

    fn remove_all_extra_values(&mut self, mut head: usize) {
        loop {
            let extra = self.remove_extra_value(head);

            if let LinkType::Extra(idx) = extra.next {
                head = idx;
            } else {
                break;
            }
        }
    }
}

#[test]
fn test_remove_all_extra_values_removes_correctly() {
    let mut list = List::new(vec![
        Link { next: LinkType::Extra(1) }, // 0th index links to 1st
        Link { next: LinkType::Extra(2) }, // 1st to 2nd
        Link { next: LinkType::None },      // 2nd is the last
    ]);

    list.remove_all_extra_values(0);
    assert_eq!(list.links.len(), 3); // All links should still be there
}

#[test]
fn test_remove_all_extra_values_stops_at_none() {
    let mut list = List::new(vec![
        Link { next: LinkType::Extra(1) }, // 0th index links to 1st
        Link { next: LinkType::Extra(2) }, // 1st to 2nd
        Link { next: LinkType::None },      // 2nd is the last
    ]);

    list.remove_all_extra_values(2);
    assert_eq!(list.links.len(), 3); // No links should be removed at head 2
}

#[test]
fn test_remove_all_extra_values_empty_list() {
    let mut list = List::new(vec![]);
    list.remove_all_extra_values(0);
    assert_eq!(list.links.len(), 0); // No links should be in the list
}

