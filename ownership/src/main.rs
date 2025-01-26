/// ```rust
/// let s: String = String::from("hello");
/// 
///  let m = s;
/// 
///  print!("{:?}",s); // This will throw an error because the ownership of s has been moved to m
/// ```
/// 
/// for a String type: it allocates a value in the heap memory and a pointer to the value in the stack memory
/// ```rust 
/// let s: String = String::from("hello"); 
/// ```
/// s contains a memory in stack that points to the real value in memory
/// ```rust
/// let m: String = s;
/// ```
/// ### m now points to the same memory as s, (shallow copy). But in rust the shallow copy does not work like legacy programming languages.
/// 
/// **Shallow copy in legacy programming languages:** the value of s is copied to m, so if you change the value of s, m will not be affected.
/// s->"hello", m->"hello". If you change s to "world", m will also be "world". As the real data it points to was affected.
/// 
/// **In rust:** shallow copy may introduce a problem of double free, where you modify the same data from two different variables. Which can cause data ambiguity.
/// 
/// **To avoid this,** rust uses the concept of ownership. When you assign a value to another variable, the ownership of the value is moved to the new variable.
/// 
/// which means that s->"hello". (without creating a new "hello"). and m really does point to the same memory as s. But the compiler would not allow you to use it.
/// 
/// this is because it would violate the ownership rule. a value can only have one owner at a time. (you can have multiple read-only references to a value, but only one owner from which you can modify the value)
/// 
/// to solve this problem, you can move the usage of s before the assignment to m. Or you can clone the value of s to m. (deep copy)
fn main() {
    let s = String::from("hello");
    println!("{:?}",s); // the problem was using s after the end of its ownership to the value "hello" in the heap.
    let m = s;
    println!("{:?}",m);
}
