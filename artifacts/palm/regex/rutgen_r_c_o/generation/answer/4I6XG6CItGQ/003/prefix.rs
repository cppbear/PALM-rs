// Answer 0

#[test]
fn test_iter_with_ac_matcher() {
    let literals = Literals::new(vec![Literal::from(vec![b'a', b'b']), Literal::from(vec![b'c', b'd'])]);
    let automaton = FullAcAutomaton::<Literal>::new(literals.patterns());
    let matcher = Matcher::AC(automaton);
    
    let searcher = LiteralSearcher::new(literals, matcher);
    let result = searcher.iter();
}

#[test]
fn test_iter_with_empty_ac_matcher() {
    let literals = Literals::new(vec![]);
    let automaton = FullAcAutomaton::<Literal>::new(literals.patterns());
    let matcher = Matcher::AC(automaton);
    
    let searcher = LiteralSearcher::new(literals, matcher);
    let result = searcher.iter();
}

