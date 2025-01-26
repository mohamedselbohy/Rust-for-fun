fn gives_ownership() -> String {
    String::from("hello")
}

pub fn main() {
    let f: String = gives_ownership();
    let (s2, len) = calculate_length(&f);
    let f: &String = s2;
    println!("{:?} {:?} from {:?}", s2, len, f);
}

fn calculate_length(s: &String) -> (&String, usize) {
    let length = s.len();
    (s, length)
}
