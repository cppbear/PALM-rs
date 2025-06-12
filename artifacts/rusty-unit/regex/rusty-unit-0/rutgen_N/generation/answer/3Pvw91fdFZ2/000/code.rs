// Answer 0

#[derive(Debug)]
struct GroupHolder {
    groups: Vec<String>,
}

impl GroupHolder {
    fn new(groups: Vec<String>) -> Self {
        GroupHolder { groups }
    }
    
    fn get(&self, i: usize) -> Option<&String> {
        self.groups.get(i)
    }
}

#[test]
fn test_index_valid() {
    let holder = GroupHolder::new(vec!["first".to_string(), "second".to_string(), "third".to_string()]);
    assert_eq!(holder.index(1), "second");
}

#[test]
#[should_panic(expected = "no group at index '3'")]
fn test_index_out_of_bounds() {
    let holder = GroupHolder::new(vec!["first".to_string(), "second".to_string()]);
    holder.index(3);
}

#[test]
#[should_panic(expected = "no group at index '0'")]
fn test_index_empty() {
    let holder = GroupHolder::new(vec![]);
    holder.index(0);
}

