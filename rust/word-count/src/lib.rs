use std::char;
use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {

    let mut hm: HashMap<String, u32> = HashMap::new();
    let words: Vec<String> = s.to_lowercase()
                              .split(|c| ! char::is_alphanumeric(c))
                              .filter(|w| ! w.is_empty())
                              .map(|s| s.to_string())
                              .collect();

    for word in words.iter() {
        let word_ = word.clone();
        let count = hm.entry(word_).or_insert(0);
        *count += 1;
    }
    return hm;
}
