// Answer 0

#[test]
fn test_append_string_zero_length() {
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(10);
    let alphabetic = Alphabetic;
    alphabetic.append_string(&mut rng, &mut string, 0);
}

#[test]
fn test_append_string_small_length() {
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(10);
    let alphabetic = Alphabetic;
    alphabetic.append_string(&mut rng, &mut string, 5);
}

#[test]
fn test_append_string_mid_length() {
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(10);
    let alphabetic = Alphabetic;
    alphabetic.append_string(&mut rng, &mut string, 512);
}

#[test]
fn test_append_string_max_length() {
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(1024);
    let alphabetic = Alphabetic;
    alphabetic.append_string(&mut rng, &mut string, 1024);
}

#[test]
fn test_append_string_with_initial_capacity() {
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(20);
    let alphabetic = Alphabetic;
    alphabetic.append_string(&mut rng, &mut string, 10);
} 

#[test]
fn test_append_string_large_initial_capacity() {
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(2048);
    let alphabetic = Alphabetic;
    alphabetic.append_string(&mut rng, &mut string, 1000);
} 

#[test]
#[should_panic]
fn test_append_string_invalid_length() {
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(10);
    let alphabetic = Alphabetic;
    alphabetic.append_string(&mut rng, &mut string, 2000);
} 

#[test]
fn test_append_string_empty_initial_string() {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    let alphabetic = Alphabetic;
    alphabetic.append_string(&mut rng, &mut string, 1);
}

