

pub fn anagrams_for<'a>(target: &str, inputs: & [&'a str]) -> Vec<&'a str> {

    fn sort_string(s: &str) -> Vec<char> {
        let mut target_chars: Vec<char> = s.to_lowercase().chars().collect();
        target_chars.sort();
        return target_chars;
    }

    let target_sorted = sort_string(&target);

    inputs.iter().cloned()
        .filter(|s| s.to_lowercase() != target.to_lowercase())
        .filter(|s| sort_string(s) == target_sorted)
        .collect()
}
