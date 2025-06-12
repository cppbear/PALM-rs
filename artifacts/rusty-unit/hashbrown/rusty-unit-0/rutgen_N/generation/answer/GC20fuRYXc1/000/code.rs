// Answer 0

#[test]
fn test_len_empty_table() {
    struct Table {
        items: usize,
    }

    struct HashTable {
        table: Table,
    }

    impl HashTable {
        fn len(&self) -> usize {
            self.table.items
        }
    }

    let hash_table = HashTable { table: Table { items: 0 } };
    assert_eq!(hash_table.len(), 0);
}

#[test]
fn test_len_non_empty_table() {
    struct Table {
        items: usize,
    }

    struct HashTable {
        table: Table,
    }

    impl HashTable {
        fn len(&self) -> usize {
            self.table.items
        }
    }

    let hash_table = HashTable { table: Table { items: 5 } };
    assert_eq!(hash_table.len(), 5);
}

