use std::io;
fn main() {
    let n: i32 = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        input.trim().parse().unwrap()
    };
    println!("Enter {} numbers", n);
    let mut a: Vec<i32> = Vec::new();
    'outer_loop: for _i in 0..n {
        let nums: Vec<i32> = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            input
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        };
        for num in nums.iter() {
            a.push(*num);
            if a.len() >= n as usize {
                break 'outer_loop;
            }
        }
    }
    for elem in a.iter() {
        print!("{}{} ", elem, if elem == a.last().unwrap() { "" } else { "," });
    }
    println!();
}
