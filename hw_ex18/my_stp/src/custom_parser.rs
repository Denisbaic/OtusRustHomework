use std::collections::HashMap;

pub fn parse_request_parameters(target: &str) -> HashMap<&str, &str> {
    let mut dictionary: HashMap<&str, &str> = HashMap::new();
    let words: Vec<&str> = target.split_whitespace().skip(1).collect();
    for word in words {
        let key_value: Vec<&str> = word.split(':').collect();
        assert_eq!(key_value.len(), 2);
        dictionary.insert(key_value[0], key_value[1]);
    }
    dictionary
}
