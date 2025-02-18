use std::collections::HashMap;
// currently _options instead of options since we don't support additional options inside the search operations.
// further options might be like case insensitivity
pub fn search<'a>(
    lineind: usize,
    pattern: &str,
    input: &'a str,
    _options: &HashMap<&str, Vec<String>>,
) -> Vec<(usize, &'a str)> {
    let lines: Vec<&str> = input.split_inclusive("\n").collect();
    let mut out: Vec<(usize, &str)> = Vec::new();
    for (ind, line) in lines.iter().enumerate() {
        if line.contains(pattern) {
            out.push((lineind as usize + ind, line));
        }
    }
    out
}
