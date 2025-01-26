use std::io;
#[derive(Debug)]
// definitely better to use String instead of &str because to let the struct own its data. Using a reference to a string would require lifetime annotations, which would complicate this code.
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let n: i32 = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        input.trim().parse().expect("not a number")
    };
    let mut i = 0;
    let mut head: Option<Box<Node>> = None;
    let mut ptr: &mut Option<Box<Node>> = &mut head;
    while i < n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let strs = input.trim().split_whitespace().collect::<Vec<&str>>();
        for st in strs.iter() {
            let num: i32 = st.parse().expect("not a number");
            let node = Box::new(Node { data: num, next: None });
            match ptr {
                None => {
                    *ptr = Some(node);
                }
                Some(ref mut p) => {
                    p.next = Some(node);
                }
            }
            ptr = &mut ptr.as_mut().unwrap().next;
            i += 1;
            if i >= n {
                break;
            }
        }
    }
        ptr = &mut head;
        while ptr.is_some() {
            match ptr {
                None => {
                    break;
                }
                Some(ref p) => {
                    print!("{} ", p.data);
                }
            }
            ptr = &mut ptr.as_mut().unwrap().next;
        }
}
