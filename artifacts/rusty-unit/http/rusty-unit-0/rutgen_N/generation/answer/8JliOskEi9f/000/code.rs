// Answer 0

#[derive(Debug)]
struct Link {
    links: Vec<i32>,
}

struct HeaderMap(Vec<Link>);

impl HeaderMap {
    fn new() -> Self {
        HeaderMap(Vec::new())
    }

    fn add_link(&mut self, link: Link) {
        self.0.push(link);
    }

    fn index_mut(&mut self, idx: usize) -> &mut Vec<i32> {
        unsafe { &mut (*self.0)[idx].links }
    }
}

#[test]
fn test_index_mut_valid_index() {
    let mut header_map = HeaderMap::new();
    header_map.add_link(Link { links: vec![1, 2, 3] });

    let links = header_map.index_mut(0);
    links.push(4);
    
    assert_eq!(links.len(), 4);
    assert_eq!(links[3], 4);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_mut_out_of_bounds() {
    let mut header_map = HeaderMap::new();
    header_map.add_link(Link { links: vec![1, 2] });

    let _ = header_map.index_mut(1); // This should panic
}

