use std::io::stdin;
fn main() {
    let mut _n: String = String::new();
    stdin().read_line(&mut _n).expect("Failed to read line");
    let mut v: Vec<i32> = Vec::new();
    let n: i32 = _n.trim().parse().expect("Please enter a number");
    let mut i = 0;
    while i < n {
        let mut _a: String = String::new();
        stdin().read_line(&mut _a).expect("Failed to read line");
        let _a: Vec<&str> = _a.split_whitespace().collect();
        for element in _a {
            v.push(element.trim().parse().expect("Failed to parse"));
            i += 1;
            if i == n {
                break;
            }
        }
    }
    for elem in v.iter() {
        print!("{} ", elem);
    }
}
