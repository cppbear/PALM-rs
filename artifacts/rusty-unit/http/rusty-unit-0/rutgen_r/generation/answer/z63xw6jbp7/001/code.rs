// Answer 0

#[derive(Default)]
struct Entry<T> {
    value: T,
    links: Option<Link>,
}

#[derive(Default)]
struct Link {
    next: usize,
}

struct Map<T> {
    entries: Vec<Entry<T>>,
    extra_values: Vec<T>,
}

struct ValueDrain<'a, T> {
    first: Option<T>,
    next: Option<dyn Iterator<Item = T> + 'a>,
    lt: std::marker::PhantomData<&'a T>,
}

impl<T> Map<T> {
    fn new(size: usize) -> Self {
        Map {
            entries: vec![Entry::default(); size],
            extra_values: Vec::new(),
        }
    }
    
    fn raw_links(&self) -> &Vec<T> {
        &self.extra_values
    }
}

#[test]
fn test_insert_occupied_mult_valid_index() {
    let mut map = Map::new(3);
    map.entries[0].value = 10;
    map.entries[0].links = Some(Link { next: 1 });

    let result: ValueDrain<i32> = map.insert_occupied_mult(0, 20);
    
    assert_eq!(result.first, Some(10));
    assert!(result.next.is_none());
}

#[test]
#[should_panic]
fn test_insert_occupied_mult_out_of_bounds() {
    let mut map = Map::new(3);
    
    // Attempting to access an out of bounds index should panic
    let _ = map.insert_occupied_mult(5, 30);
}

#[test]
fn test_insert_occupied_mult_with_link() {
    let mut map = Map::new(3);
    map.entries[1].value = 15;
    map.entries[1].links = Some(Link { next: 2 });

    let result: ValueDrain<i32> = map.insert_occupied_mult(1, 25);

    assert_eq!(result.first, Some(15));
    assert!(result.next.is_none());
}

