

pub fn anagrams_for<'a>(target: &str, inputs: & [&'a str]) -> Vec<&'a str> {

    fn sort_string(s: &str) -> Vec<char> {
        let mut target_chars: Vec<char> = s.to_lowercase().chars().collect();
        target_chars.sort();
        return target_chars;
    }

    let target_sorted = sort_string(&target);
    let mut outputs: Vec<&str> = Vec::new();

    for i in inputs.iter().filter(|&s| *s.to_lowercase() != target.to_lowercase()) {
        let i_sorted = sort_string(i);
        if i_sorted == target_sorted {
            outputs.push(i);
        }
    }

    return outputs;
}
