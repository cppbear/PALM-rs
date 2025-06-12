// Answer 0

#[derive(Default)]
struct Table {
    items: usize,
    growth_left: usize,
}

struct HashMap {
    table: Table,
}

impl HashMap {
    pub fn new(items: usize, growth_left: usize) -> Self {
        Self {
            table: Table {
                items,
                growth_left,
            },
        }
    }

    pub fn capacity(&self) -> usize {
        self.table.items + self.table.growth_left
    }
}

#[test]
fn test_capacity_with_no_items() {
    let map = HashMap::new(0, 10);
    assert_eq!(map.capacity(), 10);
}

#[test]
fn test_capacity_with_items() {
    let map = HashMap::new(5, 5);
    assert_eq!(map.capacity(), 10);
}

#[test]
fn test_capacity_with_large_growth() {
    let map = HashMap::new(3, 100);
    assert_eq!(map.capacity(), 103);
}

#[test]
fn test_capacity_with_zero_growth() {
    let map = HashMap::new(8, 0);
    assert_eq!(map.capacity(), 8);
}

