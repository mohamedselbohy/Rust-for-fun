pub fn main(){
    let a = [1, 2, 3, 4, 5];
    let n = 3; let begin= 1;
    let slice = &a[begin..n];
    println!("{:?}", slice[n-begin-1]);
}
