use std::char;
use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {

    s.to_lowercase()
     .split(|c| ! char::is_alphanumeric(c))
     .filter(|w| ! w.is_empty())
     .map(|s| s.to_string())
     .fold(HashMap::new(), |mut hm, word| {
        *hm.entry(word).or_insert(0) += 1; // this looks odd to me, got it from http://stackoverflow.com/questions/31884309/how-to-fold-using-a-hashmap-as-an-accumulator
        hm
     })

}
