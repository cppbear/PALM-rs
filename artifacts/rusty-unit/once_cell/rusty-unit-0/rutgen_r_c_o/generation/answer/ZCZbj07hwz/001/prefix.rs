// Answer 0

#[test]
fn test_force_mut_with_non_primitive_type() {
    struct MyStruct {
        value: i32,
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| MyStruct { value: 42 })),
    };

    let result: &mut MyStruct = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_another_non_primitive_type() {
    struct AnotherStruct {
        text: String,
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| AnotherStruct { text: String::from("Hello") })),
    };

    let result: &mut AnotherStruct = Lazy::force_mut(&mut lazy);
} 

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_when_poisoned() {
    struct PoisonedStruct;

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };

    let _result: &mut PoisonedStruct = Lazy::force_mut(&mut lazy);
} 

#[test]
fn test_force_mut_with_different_return_value() {
    struct DifferentStruct {
        number: f64,
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| DifferentStruct { number: 3.14 })),
    };

    let result: &mut DifferentStruct = Lazy::force_mut(&mut lazy);
} 

#[test]
fn test_force_mut_multiple_times() {
    struct RepeatedStruct {
        count: u32,
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| RepeatedStruct { count: 10 })),
    };

    let result1: &mut RepeatedStruct = Lazy::force_mut(&mut lazy);
    let result2: &mut RepeatedStruct = Lazy::force_mut(&mut lazy);
}

