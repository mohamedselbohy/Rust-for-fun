fn largest_char<T>(list: &[T]) -> T 
where T: PartialOrd, T: Copy // Here you specify that only types that implement the PartialOrd and Copy traits can be used with this function
{
    let mut largest = &list[0];
    for item in list {
        if item > largest { // Here you have to tell the compiler that T should also implement the PartialOrd trait in order to be able to use > operator
            largest = item;
        }
    }
    *largest // because largest is a reference, we need to dereference it
            // Since T implements Copy, we can return a copy of the value i.e. ret = *largest can only compile if we can move the value out of the reference without altering ownership
            // this is only possible if T implements Copy. This also applies to ret = largest without needing to clone the value because it copies it by default since the value is present in the stack.
}

fn main() {

    let a = &"abc";
    let b = *a; // This works because &str implements Copy
    println!("{} {}",b ,a);
    println!("{}",largest_char("niuabiufuy".as_bytes()) as char); // works because char implements Copy (char is primitive) and PartialOrd
    println!("{}", largest_char(&["abcds","az","a","abc"])); // Still works because &str implements Copy (to your surprise str is primitive too) and PartialOrd
    // uncomment the line below to let the compiler insult you.
    // println!("{}", largest_char(&["abcds".to_owned(),"az".to_owned(),"a".to_owned(),"abc".to_owned()])); // This will not work because String does not implement Copy
}