// Answer 0

#[test]
fn test_into_table_valid() {
    struct HashTable<T, A> {
        data: Vec<T>,
    }

    struct AbsentEntry<'a, T, A> {
        table: &'a mut HashTable<T, A>,
    }

    let mut table = HashTable { data: vec![1, 2, 3] };
    let absent_entry = AbsentEntry { table: &mut table };

    let result: &mut HashTable<i32, ()> = absent_entry.into_table();
    assert_eq!(result.data.len(), 3);
}

#[test]
fn test_into_table_empty_table() {
    struct HashTable<T, A> {
        data: Vec<T>,
    }

    struct AbsentEntry<'a, T, A> {
        table: &'a mut HashTable<T, A>,
    }

    let mut table = HashTable { data: Vec::<i32>::new() };
    let absent_entry = AbsentEntry { table: &mut table };

    let result: &mut HashTable<i32, ()> = absent_entry.into_table();
    assert!(result.data.is_empty());
}

