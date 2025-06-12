// Answer 0

#[derive(Debug)]
struct Link {
    pub links: Vec<String>,
}

struct Container(Vec<Link>);

impl Container {
    fn new(links: Vec<Link>) -> Self {
        Container(links)
    }
}

impl Container {
    type Output = Vec<String>;

    fn index(&self, idx: usize) -> &Self::Output {
        unsafe { &(*self.0)[idx].links }
    }
}

#[test]
fn test_index_valid() {
    let links1 = Link { links: vec!["link1".to_string(), "link2".to_string()] };
    let links2 = Link { links: vec!["link3".to_string()] };
    let container = Container::new(vec![links1, links2]);

    let result = container.index(0);
    assert_eq!(result, &vec!["link1".to_string(), "link2".to_string()]);
}

#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    let links = Link { links: vec!["link1".to_string()] };
    let container = Container::new(vec![links]);

    let _result = container.index(1); // This should panic
}

