// Answer 0

#[test]
fn test_capacity_with_zero_items_and_growth_left() {
    struct Map {
        table: Table,
    }

    struct Table {
        items: usize,
        growth_left: usize,
    }

    let map = Map {
        table: Table {
            items: 0,
            growth_left: 0,
        },
    };

    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_capacity_with_non_zero_items_and_zero_growth_left() {
    struct Map {
        table: Table,
    }

    struct Table {
        items: usize,
        growth_left: usize,
    }

    let map = Map {
        table: Table {
            items: 10,
            growth_left: 0,
        },
    };

    assert_eq!(map.capacity(), 10);
}

#[test]
fn test_capacity_with_zero_items_and_non_zero_growth_left() {
    struct Map {
        table: Table,
    }

    struct Table {
        items: usize,
        growth_left: usize,
    }

    let map = Map {
        table: Table {
            items: 0,
            growth_left: 5,
        },
    };

    assert_eq!(map.capacity(), 5);
}

#[test]
fn test_capacity_with_large_values() {
    struct Map {
        table: Table,
    }

    struct Table {
        items: usize,
        growth_left: usize,
    }

    let map = Map {
        table: Table {
            items: usize::MAX - 1,
            growth_left: 1,
        },
    };

    assert_eq!(map.capacity(), usize::MAX);
}

#[test]
fn test_capacity_with_large_items_and_zero_growth() {
    struct Map {
        table: Table,
    }

    struct Table {
        items: usize,
        growth_left: usize,
    }

    let map = Map {
        table: Table {
            items: usize::MAX,
            growth_left: 0,
        },
    };

    assert_eq!(map.capacity(), usize::MAX);
}

