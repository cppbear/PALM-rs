// Answer 0

#[test]
fn test_append_string_case_1() {
    let slice: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
                           'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 
                           'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 
                           'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 
                           'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 
                           'Y', 'Z', '!', '@', '#', '$', '%', '^', '&', '*', 
                           '(', ')', '-', '=', '+', '{', '}', '[', ']', '|', 
                           '\\', ':', ';', '"', '\'', '<', '>', ',', '.', '/', 
                           '?', '`', '~', ' ', '0', '1', '2', '3', '4', '5', 
                           '6', '7', '8', '9'];
    let num_choices = NonZeroUsize::new(1).unwrap();
    let range = UniformUsize { low: 0, range: 200, thresh: 200, mode64: false };
    
    let chooser = Choose { slice, range, num_choices };
    
    let mut rng = rand::thread_rng();
    let mut output_string = String::new();
    chooser.append_string(&mut rng, &mut output_string, 100);
}

#[test]
fn test_append_string_case_2() {
    let slice: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
                           'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
                           'U', 'V', 'W', 'X', 'Y', 'Z', '!', '@', '#', '$',
                           '%', '^', '&', '*', '(', ')', '-', '=', '+', 
                           '{', '}', '[', ']', '|', '\\', ':', ';', '"', 
                           '\'', '<', '>', ',', '.', '/', '?', '`', '~', 
                           ' ', '0', '1', '2', '3', '4', '5', '6', '7', 
                           '8', '9'];
    let num_choices = NonZeroUsize::new(1).unwrap();
    let range = UniformUsize { low: 0, range: 200, thresh: 200, mode64: false };
    
    let chooser = Choose { slice, range, num_choices };
    
    let mut rng = rand::thread_rng();
    let mut output_string = String::new();
    chooser.append_string(&mut rng, &mut output_string, 150);
}

