// Answer 0

#[derive(Debug, Default)]
struct HeaderMap<T> {
    headers: std::collections::HashMap<HeaderName, T>,
}

impl<T> HeaderMap<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (HeaderName, T)>,
    {
        for (key, value) in iter {
            self.headers.insert(key, value);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct HeaderName(String);

impl HeaderMap<String> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (HeaderName, String)>,
    {
        let mut map = HeaderMap::default();
        map.extend(iter);
        map
    }
}

#[test]
fn test_from_iter_empty() {
    let empty_iter: Vec<(HeaderName, String)> = Vec::new();
    let map: HeaderMap<String> = HeaderMap::from_iter(empty_iter);
    assert!(map.headers.is_empty());
}

#[test]
fn test_from_iter_single() {
    let headers = vec![(HeaderName("Content-Type".into()), "text/html".into())];
    let map: HeaderMap<String> = HeaderMap::from_iter(headers);
    assert_eq!(map.headers.len(), 1);
    assert_eq!(map.headers.get(&HeaderName("Content-Type".into())), Some(&"text/html".into()));
}

#[test]
fn test_from_iter_multiple() {
    let headers = vec![
        (HeaderName("Accept".into()), "text/html".into()),
        (HeaderName("User-Agent".into()), "Mozilla/5.0".into()),
    ];
    let map: HeaderMap<String> = HeaderMap::from_iter(headers);
    assert_eq!(map.headers.len(), 2);
    assert_eq!(map.headers.get(&HeaderName("Accept".into())), Some(&"text/html".into()));
    assert_eq!(map.headers.get(&HeaderName("User-Agent".into())), Some(&"Mozilla/5.0".into()));
}

