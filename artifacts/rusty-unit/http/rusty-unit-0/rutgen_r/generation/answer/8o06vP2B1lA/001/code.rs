// Answer 0

#[derive(Debug)]
struct Link {
    links: String,
}

struct TestStruct(Vec<Link>);

impl TestStruct {
    fn index(&self, idx: usize) -> &String {
        unsafe { &(*self.0)[idx].links }
    }
}

#[test]
fn test_index_valid() {
    let links = vec![Link { links: String::from("link1") }, Link { links: String::from("link2") }];
    let test_struct = TestStruct(links);
    assert_eq!(test_struct.index(0), "link1");
    assert_eq!(test_struct.index(1), "link2");
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_index_out_of_bounds_low() {
    let links = vec![Link { links: String::from("link1") }];
    let test_struct = TestStruct(links);
    // This should panic because there is no index -1
    let _ = test_struct.index(1);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_index_out_of_bounds_high() {
    let links = vec![Link { links: String::from("link1") }];
    let test_struct = TestStruct(links);
    // This should panic because the index is out of bounds
    let _ = test_struct.index(2);
}

