pub fn main() {
    let mut s: String = String::from("hello");
    change(&mut s);
    let s2: &mut String = &mut s;
    s2.push_str(", my dear");
    println!("{:?}", s2);
    let a: &mut String = &mut s;
    println!("{:?}", a);
    let b: &mut String = &mut s; // Rust enforces for references that at most one mutable reference can be active at a time. Or can have multiple immutable references without any mutable references at all.
    print!("{:?}", b); // This will throw an error because we have already borrowed s as mutable reference in a
                       //println!("{:?} {:?}", a, b);
}

fn change(s: &mut String) {
    s.push_str(" world");
}
