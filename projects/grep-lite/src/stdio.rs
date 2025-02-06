use std::collections::HashMap;
// currently _options instead of options since we don't support additional options inside the search operations.
// further options might be like case insensitivity
pub fn search<'a>(pattern: &str, input: &'a str , _options: &HashMap<&str, Vec<String>>) -> Vec<(usize, &'a str)>{
    //let input = { let mut _in = String::new(); io::stdin().read_line(&mut _in).expect("Couldn't read text"); _in };
    let lines: Vec<&str> = input.split_inclusive("\n").collect();
    let mut out: Vec<(usize, &str)> = Vec::new();
    for (ind, line) in lines.iter().enumerate(){
        if line.contains(pattern) {
            out.push((ind+1, line));
        }
    }
    println!("{:?}", input);
    out
}