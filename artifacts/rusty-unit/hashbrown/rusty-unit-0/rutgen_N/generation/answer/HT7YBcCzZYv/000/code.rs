// Answer 0

#[test]
fn test_find_or_find_insert_slot_found() {
    struct Dummy {
        value: i32,
    }

    let mut table = RawTable::new(); // Assuming RawTable::new creates an empty table
    table.insert(Dummy { value: 1 }, |v| v.value.hash(), |v| v.value == 1); // Inserting a dummy value

    let result = table.find_or_find_insert_slot(
        1.hash(), 
        |item| item.value == 1, 
        |item| item.value.hash()
    );

    assert!(result.is_ok());
    let bucket = result.unwrap();
    assert_eq!(bucket.as_ref().value, 1);
}

#[test]
fn test_find_or_find_insert_slot_not_found() {
    struct Dummy {
        value: i32,
    }

    let mut table = RawTable::new(); // Assuming RawTable::new creates an empty table

    let result = table.find_or_find_insert_slot(
        2.hash(),
        |item| item.value == 2,
        |item| item.value.hash(),
    );

    assert!(result.is_err());
    if let Err(slot) = result {
        assert_eq!(slot, InsertSlot::new()); // Validate slot structure as needed
    }
}

#[test]
fn test_find_or_find_insert_slot_resize() {
    // Test behavior when resizing is needed
    struct Dummy {
        value: i32,
    }

    let mut table = RawTable::new(); // Assuming RawTable::new creates an empty table
    for i in 0..10 { // Insert multiple items to trigger resize
        table.insert(Dummy { value: i }, |v| v.value.hash(), |v| v.value == i);
    }

    let result = table.find_or_find_insert_slot(
        10.hash(),
        |item| item.value == 10,
        |item| item.value.hash(),
    );

    assert!(result.is_err());
}

