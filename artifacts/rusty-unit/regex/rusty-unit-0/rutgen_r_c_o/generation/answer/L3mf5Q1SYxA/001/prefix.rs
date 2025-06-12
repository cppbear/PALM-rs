// Answer 0

#[test]
fn test_locations_empty_captures() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly::new()),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let _locations = regex.locations();
}

#[test]
fn test_locations_even_captures() {
    let mut exec = Exec {
        ro: Arc::new(ExecReadOnly::new()),
        cache: CachedThreadLocal::new(),
    };
    let captures_len = 2; // Even and within range
    for _ in 0..captures_len {
        exec.capture_names.push(Some("name".to_string()));
    }
    let regex = Regex(exec);
    let _locations = regex.locations();
}

#[test]
fn test_locations_maximum_even_captures() {
    let mut exec = Exec {
        ro: Arc::new(ExecReadOnly::new()),
        cache: CachedThreadLocal::new(),
    };
    let captures_len = 65534; // Maximum even number within range
    for _ in 0..captures_len {
        exec.capture_names.push(Some("name".to_string()));
    }
    let regex = Regex(exec);
    let _locations = regex.locations();
}

#[test]
#[should_panic]
fn test_locations_exceeding_captures() {
    let mut exec = Exec {
        ro: Arc::new(ExecReadOnly::new()),
        cache: CachedThreadLocal::new(),
    };
    let captures_len = 65536; // Exceeding the maximum
    for _ in 0..captures_len {
        exec.capture_names.push(Some("name".to_string()));
    }
    let regex = Regex(exec);
    let _locations = regex.locations();
}

#[test]
#[should_panic]
fn test_locations_odd_captures() {
    let mut exec = Exec {
        ro: Arc::new(ExecReadOnly::new()),
        cache: CachedThreadLocal::new(),
    };
    let captures_len = 3; // Odd number of captures
    for _ in 0..captures_len {
        exec.capture_names.push(Some("name".to_string()));
    }
    let regex = Regex(exec);
    let _locations = regex.locations();
}

