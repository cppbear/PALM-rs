// Answer 0

#[test]
fn test_new_with_conditions() {
    let literals = vec![
        vec![b'a'], 
        vec![b'b'], 
        vec![b'c'], 
        vec![b'd'], 
        vec![b'e'], 
        vec![b'f'], 
        vec![b'g'], 
        vec![b'h'],
        vec![b'i'], 
        vec![b'j'], 
        vec![b'k'], 
        vec![b'l'], 
        vec![b'm'], 
        vec![b'n'], 
        vec![b'o'], 
        vec![b'p'], 
        vec![b'q'], 
        vec![b'r'], 
        vec![b's'], 
        vec![b't'], 
        vec![b'u'], 
        vec![b'v'], 
        vec![b'w'], 
        vec![b'x'], 
        vec![b'y'], 
        vec![b'z'], 
        vec![b'1'], 
        vec![b'2'], 
        vec![b'3'], 
        vec![b'4'], 
        vec![b'5'], 
        vec![b'6'], 
        vec![b'7'], 
        vec![b'8'], 
        vec![b'9'],
    ];
    
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a'],
        complete: false,
        all_ascii: false,
    };

    let lits = Literals::from(literals);
    
    Matcher::new(&lits, sset);
}

